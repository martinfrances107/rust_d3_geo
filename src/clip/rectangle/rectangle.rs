use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::Float;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::clip::buffer::Buffer as ClipBuffer;
use crate::clip::intersection::Intersection;
use crate::clip::line_elem::LineElem;
use crate::clip::line_fn::line as clip_line;
use crate::clip::rejoin::rejoin as clip_rejoin;
use crate::clip::rejoin::CompareIntersectionsFn;
use crate::clip::InterpolateFn;
use crate::math::EPSILON;
use crate::path::Result;
use crate::path::ResultEnum;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

///A primitive type used to for a PostClipNode pipeline stage.
#[derive(Clone, Debug)]
pub struct Rectangle<T>
where
    // PR: Raw<T>,
    T: CoordFloat,
{
    buffer_stream: ClipBuffer<T>,
    clean: bool,
    clip_min: T,
    clip_max: T,
    epsilon: T,
    first: bool,

    x0: T,
    y0: T,
    x1: T,
    y1: T,

    polygon: Option<Vec<Vec<Coordinate<T>>>>,
    segments: Option<VecDeque<VecDeque<Vec<LineElem<T>>>>>,

    // first point.
    x__: T,
    y__: T,
    v__: bool,
    // previous point.
    x_: T,
    y_: T,
    v_: bool,

    use_line_point: bool,
    use_buffer_stream: bool,
}

impl<T> Rectangle<T>
where
    // PR: Raw<T>,
    T: 'static + CoordFloat + FloatConst,
{
    #[inline]
    pub(crate) fn new(x0: T, y0: T, x1: T, y1: T) -> Rectangle<T> {
        Self {
            buffer_stream: ClipBuffer::<T>::default(),
            first: false,
            clean: false,
            clip_max: T::from(1e9).unwrap(),
            clip_min: -T::from(1e9).unwrap(),
            epsilon: T::from(EPSILON).unwrap(),

            x0,
            y0,
            x1,
            y1,

            // pr_pd: PhantomData::new(),
            polygon: None,
            segments: None,

            // first point.
            x__: T::nan(),
            y__: T::nan(),
            v__: false,

            // previous point.
            x_: T::nan(),
            y_: T::nan(),
            v_: false,
            use_line_point: false,
            use_buffer_stream: false,
        }
    }

    #[inline(always)]
    fn visible(&self, p: &Coordinate<T>) -> bool {
        self.x0 <= p.x && p.x <= self.x1 && self.y0 <= p.y && p.y <= self.y1
    }

    fn gen_corner(&self) -> Box<dyn Fn(&Coordinate<T>, &T) -> i8> {
        let x0 = self.x0;
        let y0 = self.y0;
        let x1 = self.x1;
        let epsilon = T::from(EPSILON).unwrap();
        Box::new(move |p: &Coordinate<T>, direction: &T| -> i8 {
            if (p.x - x0).abs() < epsilon {
                if direction > &T::zero() {
                    0
                } else {
                    3
                }
            } else if (p.x - x1).abs() < epsilon {
                if direction > &T::zero() {
                    2
                } else {
                    1
                }
            } else if (p.y - y0).abs() < epsilon {
                if direction > &T::zero() {
                    1
                } else {
                    0
                }
            } else if direction > &T::zero() {
                3
            } else {
                2
            }
        })
    }

    fn polygon_inside(&self) -> bool {
        let mut winding = 0;

        if let Some(polygon) = &self.polygon {
            for p in polygon.iter() {
                let mut point = p[0];
                let mut a0;
                let mut a1;
                let mut b0 = point.x;
                let mut b1 = point.y;
                for ring in p.iter().skip(1) {
                    a0 = b0;
                    a1 = b1;
                    point = *ring;
                    b0 = point.x;
                    b1 = point.y;

                    if a1 <= self.y1 {
                        if b1 > self.y1 && (b0 - a0) * (self.y1 - a1) > (b1 - a1) * (self.x0 - a0) {
                            winding += 1;
                        }
                    } else if b1 <= self.y1
                        && (b0 - a0) * (self.y1 - a1) < (b1 - a1) * (self.x0 - a0)
                    {
                        winding -= 1
                    }
                }
            }
        }
        !winding.is_zero()
    }

    /// Warning from JS a, b are LineElem.
    fn gen_compare_point(&self) -> Box<dyn Fn(&Coordinate<T>, &Coordinate<T>) -> Ordering> {
        let corner = self.gen_corner();
        Box::new(move |a: &Coordinate<T>, b: &Coordinate<T>| -> Ordering {
            let ca = corner(a, &T::one());
            let cb = corner(b, &T::one());
            if ca != cb {
                if (ca - cb) > 0 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else {
                let diff = match ca {
                    0 => b.y - a.y,
                    1 => a.x - b.x,
                    2 => a.y - b.y,
                    _ => b.x - a.x,
                };
                if diff > T::zero() {
                    Ordering::Greater
                } else if diff < T::zero() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        })
    }
}

impl<EP, SINK, T> StreamNode<EP, Rectangle<T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    #[inline]
    fn default_point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.raw.visible(p) {
            if self.raw.use_buffer_stream {
                self.raw.buffer_stream.point(p, m);
            } else {
                self.sink.point(p, m);
            }
        }
    }

    fn line_point(&mut self, p_in: &Coordinate<T>, m: Option<u8>) {
        let mut p = *p_in;
        let v = self.raw.visible(&p);

        // Divergence from JS.
        // Here the JS version pushes to a variable called 'ring'
        // which in array terms is always the last item on the polygon array
        if let Some(polygon) = &mut self.raw.polygon {
            if let Some(last) = polygon.last_mut() {
                last.push(p);
            }
        }

        if self.raw.first {
            self.raw.x__ = p.x;
            self.raw.y__ = p.y;
            self.raw.v__ = v;
            self.raw.first = false;
            if v {
                if self.raw.use_buffer_stream {
                    self.raw.buffer_stream.line_start();
                    self.raw.buffer_stream.point(&p, None);
                } else {
                    self.sink.line_start();
                    self.sink.point(&p, None);
                };
            }
        } else if v && self.raw.v_ {
            if self.raw.use_buffer_stream {
                self.raw.buffer_stream.point(&p, m);
            } else {
                self.sink.point(&p, m);
            }
        } else {
            self.raw.x_ = Float::max(
                self.raw.clip_min,
                Float::min(self.raw.clip_max, self.raw.x_),
            );
            self.raw.y_ = Float::max(
                self.raw.clip_min,
                Float::min(self.raw.clip_max, self.raw.y_),
            );

            let mut a = [self.raw.x_, self.raw.y_];
            p.x = T::max(self.raw.clip_min, T::min(self.raw.clip_max, p.x));
            p.y = T::max(self.raw.clip_min, T::min(self.raw.clip_max, p.y));
            let mut b = [p.x, p.y];
            if clip_line(
                &mut a,
                &mut b,
                self.raw.x0,
                self.raw.y0,
                self.raw.x1,
                self.raw.y1,
            ) {
                if !self.raw.v_ {
                    if self.raw.use_buffer_stream {
                        self.raw.buffer_stream.line_start();
                        self.raw
                            .buffer_stream
                            .point(&Coordinate { x: a[0], y: a[1] }, None);
                    } else {
                        self.sink.line_start();
                        self.sink.point(&Coordinate { x: a[0], y: a[1] }, None);
                    }
                }
                if self.raw.use_buffer_stream {
                    self.raw
                        .buffer_stream
                        .point(&Coordinate { x: b[0], y: b[1] }, None);
                } else {
                    self.sink.point(&Coordinate { x: b[0], y: b[1] }, None);
                }
                if !v {
                    if self.raw.use_buffer_stream {
                        self.raw.buffer_stream.line_end();
                    } else {
                        self.sink.line_end();
                    }
                    self.raw.clean = false;
                }
            } else if v {
                if self.raw.use_buffer_stream {
                    self.raw.buffer_stream.line_start();
                    self.raw.buffer_stream.point(&p, None);
                } else {
                    self.sink.line_start();
                    self.sink.point(&p, None);
                }
                self.raw.clean = false;
            }
        }

        self.raw.x_ = p.x;
        self.raw.y_ = p.y;
        self.raw.v_ = v;
    }

    fn gen_interpolate(&self) -> InterpolateFn<SINK, T> {
        // Is capturing here a good thing.
        let x0 = self.raw.x0;
        let y0 = self.raw.y0;
        let x1 = self.raw.x1;
        let y1 = self.raw.y1;

        let compare_point = self.raw.gen_compare_point();
        let corner = self.raw.gen_corner();
        Rc::new(
            move |from: Option<Coordinate<T>>,
                  to: Option<Coordinate<T>>,
                  direction: T,
                  mut stream: SINK| {
                let mut a;
                let a1;
                let direction_i8: i8 = T::to_i8(&direction).unwrap();
                match (to, from) {
                    (Some(to), Some(from)) => {
                        a = corner(&from, &direction);
                        a1 = corner(&to, &direction);
                        let mut s_mut = stream;
                        let cp = compare_point(&from, &to) < Ordering::Less;
                        let is_direction = direction > T::zero();
                        // logical exor: cp ^^ is_direction
                        if a != a1 || (cp && !is_direction) || (!cp && is_direction) {
                            loop {
                                let p = Coordinate {
                                    x: if a == 0 || a == 3 { x0 } else { x1 },
                                    y: if a > 1 { y1 } else { y0 },
                                };
                                s_mut.point(&p, None);

                                a = (a + direction_i8 + 4) % 4;
                                if a == a1 {
                                    break;
                                }
                            }
                        }
                    }
                    (Some(to), None) => {
                        stream.point(&to, None);
                    }
                    _ => {
                        panic!("did not expect only from and no to .. or Nothing at all Does the JS version get here?");
                    }
                }
            },
        )
    }
}

impl<EP, SINK, T> Stream for StreamNode<EP, Rectangle<T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.sink.get_endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.raw.use_line_point {
            self.line_point(p, m);
        } else {
            self.default_point(p, m);
        }
    }

    // Buffer geometry within a polygon and then clip it en masse.
    fn polygon_start(&mut self) {
        // activeStream = bufferStream,
        self.raw.use_buffer_stream = true;

        self.raw.segments = Some(VecDeque::new());
        self.raw.polygon = Some(Vec::new());
        self.raw.clean = true;
    }

    fn polygon_end(&mut self) {
        let start_inside = self.raw.polygon_inside();
        let clean_inside = self.raw.clean && start_inside;

        // Performance if all lengths are known. Can I flatern into a
        // array of arrays or something that implies a contigious block of memory.
        let merged_segments = self
            .raw
            .segments
            .clone()
            .unwrap()
            .into_iter()
            .flatten()
            .collect::<Vec<Vec<LineElem<T>>>>();
        let num_visible_elements = merged_segments.len();

        let visible = !num_visible_elements.is_zero();

        if clean_inside || visible {
            {
                self.sink.polygon_start();
            }

            let interpolate_fn: InterpolateFn<SINK, T> = self.gen_interpolate();
            if clean_inside {
                // let mut sb = self;
                self.sink.line_start();
                interpolate_fn(None, None, T::one(), self.sink.clone());
                self.sink.line_end();
            }

            let compare_point = self.raw.gen_compare_point();
            let compare_intersection: CompareIntersectionsFn<T> = Box::new(
                move |a: &Rc<RefCell<Intersection<T>>>,
                      b: &Rc<RefCell<Intersection<T>>>|
                      -> Ordering {
                    compare_point(&a.borrow().x.p, &b.borrow().x.p)
                },
            );

            if visible {
                clip_rejoin(
                    &merged_segments,
                    compare_intersection,
                    start_inside,
                    interpolate_fn,
                    self.sink.clone(),
                );
            }

            self.sink.polygon_end();
        }
    }

    fn line_start(&mut self) {
        self.raw.use_line_point = true;
        match &mut self.raw.polygon {
            Some(polygon) => polygon.push(Vec::new()),
            None => {}
        }
        self.raw.first = true;
        self.raw.v_ = false;
        self.raw.x_ = T::nan();
        self.raw.y_ = T::nan();
    }

    fn line_end(&mut self) {
        if self.raw.segments.is_some() {
            self.line_point(
                &Coordinate {
                    x: self.raw.x__,
                    y: self.raw.y__,
                },
                None,
            );
            if self.raw.v__ && self.raw.v_ {
                self.raw.buffer_stream.rejoin();
            }

            if let Some(ResultEnum::BufferOutput(result)) = self.raw.buffer_stream.result() {
                self.raw.segments.as_mut().unwrap().push_back(result);
            }
        }
        self.raw.use_line_point = false;
        if self.raw.v_ {
            if self.raw.use_buffer_stream {
                self.raw.buffer_stream.line_end();
            } else {
                self.sink.line_end();
            }
        }
    }
}
