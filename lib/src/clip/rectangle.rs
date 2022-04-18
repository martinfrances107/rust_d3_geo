use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::Float;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::math::EPSILON;
use crate::path::Result;
use crate::projection::builder::PostClipNode;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::buffer::Buffer as ClipBuffer;
use super::intersection::Intersection;
use super::line_elem::LineElem;
use super::line_fn::line as clip_line;
use super::rejoin::rejoin as clip_rejoin;
use super::rejoin::CompareIntersectionsFn;
use super::Interpolator as InterpolatorTrait;

///A primitive type used to for a PostClipNode pipeline stage.
#[derive(Clone)]
pub struct Rectangle<EP, SINK, STATE, T>
where
    T: CoordFloat,
{
    state: STATE,
    p_ep: PhantomData<EP>,
    p_sink: PhantomData<SINK>,
    buffer_stream: ClipBuffer<T>,
    clean: bool,
    clip_min: T,
    clip_max: T,
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
    epsilon: T,
}

impl<EP, SINK, T> Debug for Rectangle<EP, SINK, Connected<EP>, T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointRadiusEnum").finish()
    }
}

impl<EP, SINK, T> Debug for Rectangle<EP, SINK, Unconnected, T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointRadiusEnum").finish()
    }
}

impl<EP, SINK, T> Rectangle<EP, SINK, Unconnected, T>
where
    T: 'static + CoordFloat + FloatConst,
{
    #[inline]
    pub(crate) fn new(x0: T, y0: T, x1: T, y1: T) -> Rectangle<EP, SINK, Unconnected, T> {
        Self {
            state: Unconnected,
            p_ep: PhantomData::<EP>,
            p_sink: PhantomData::<SINK>,
            buffer_stream: ClipBuffer::<T>::default(),
            first: false,
            clean: false,
            clip_max: T::from(1e9_f64).unwrap(),
            clip_min: -T::from(1e9_f64).unwrap(),

            x0,
            y0,
            x1,
            y1,

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
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<EP, SINK, STATE, T> PostClipNode for Rectangle<EP, SINK, STATE, T> where T: CoordFloat {}
impl<EP, SINK, T> Rectangle<EP, SINK, Connected<SINK>, T>
where
    SINK: Stream<EP = EP, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    #[inline(always)]
    fn visible(&self, p: &Coordinate<T>) -> bool {
        self.x0 <= p.x && p.x <= self.x1 && self.y0 <= p.y && p.y <= self.y1
    }

    // fn gen_corner(&self) -> Box<dyn Fn(&Coordinate<T>, &T) -> i8> {
    //     let x0 = self.x0;
    //     let y0 = self.y0;
    //     let x1 = self.x1;
    //     let epsilon = T::from(EPSILON).unwrap();
    //     Box::new(|p: &Coordinate<T>, direction: &T| -> i8 {
    //         if (p.x - x0).abs() < epsilon {
    //             if direction > &T::zero() {
    //                 0
    //             } else {
    //                 3
    //             }
    //         } else if (p.x - x1).abs() < epsilon {
    //             if direction > &T::zero() {
    //                 2
    //             } else {
    //                 1
    //             }
    //         } else if (p.y - y0).abs() < epsilon {
    //             if direction > &T::zero() {
    //                 1
    //             } else {
    //                 0
    //             }
    //         } else if direction > &T::zero() {
    //             3
    //         } else {
    //             2
    //         }
    //     })
    // }

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

    // Warning from JS a, b are LineElem.
    // fn gen_compare_point(&self) -> Box<dyn Fn(&Coordinate<T>, &Coordinate<T>) -> Ordering> {
    //     let corner = self.gen_corner();
    //     Box::new(move |a: &Coordinate<T>, b: &Coordinate<T>| -> Ordering {
    //         let ca = corner(a, &T::one());
    //         let cb = corner(b, &T::one());
    //         if ca != cb {
    //             if (ca - cb) > 0 {
    //                 Ordering::Greater
    //             } else {
    //                 Ordering::Less
    //             }
    //         } else {
    //             let diff = match ca {
    //                 0 => b.y - a.y,
    //                 1 => a.x - b.x,
    //                 2 => a.y - b.y,
    //                 _ => b.x - a.x,
    //             };
    //             if diff > T::zero() {
    //                 Ordering::Greater
    //             } else if diff < T::zero() {
    //                 Ordering::Less
    //             } else {
    //                 Ordering::Equal
    //             }
    //         }
    //     })
    // }

    // Warning from JS a, b are LineElem.
    // fn compare_point(&mut self, a: &Coordinate<T>, b: &Coordinate<T>) -> Ordering {
    //     let ca = self.corner(a, &T::one());
    //     let cb = self.corner(b, &T::one());
    //     if ca != cb {
    //         if (ca - cb) > 0 {
    //             Ordering::Greater
    //         } else {
    //             Ordering::Less
    //         }
    //     } else {
    //         let diff = match ca {
    //             0 => b.y - a.y,
    //             1 => a.x - b.x,
    //             2 => a.y - b.y,
    //             _ => b.x - a.x,
    //         };
    //         if diff > T::zero() {
    //             Ordering::Greater
    //         } else if diff < T::zero() {
    //             Ordering::Less
    //         } else {
    //             Ordering::Equal
    //         }
    //     }
    // }
}

impl<EP, SINK, T> Rectangle<EP, SINK, Connected<SINK>, T>
where
    SINK: Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    #[inline]
    fn default_point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.visible(p) {
            if self.use_buffer_stream {
                self.buffer_stream.point(p, m);
            } else {
                self.state.sink.point(p, m);
            }
        }
    }

    fn line_point(&mut self, p_in: &Coordinate<T>, m: Option<u8>) {
        let mut p = *p_in;
        let v = self.visible(&p);

        // Divergence from JS.
        // Here the JS version pushes to a variable called 'ring'
        // which in array terms is always the last item on the polygon array
        if let Some(polygon) = &mut self.polygon {
            if let Some(last) = polygon.last_mut() {
                last.push(p);
            }
        }

        if self.first {
            self.x__ = p.x;
            self.y__ = p.y;
            self.v__ = v;
            self.first = false;
            if v {
                if self.use_buffer_stream {
                    self.buffer_stream.line_start();
                    self.buffer_stream.point(&p, None);
                } else {
                    self.state.sink.line_start();
                    self.state.sink.point(&p, None);
                };
            }
        } else if v && self.v_ {
            if self.use_buffer_stream {
                self.buffer_stream.point(&p, m);
            } else {
                self.state.sink.point(&p, m);
            }
        } else {
            self.x_ = Float::max(self.clip_min, Float::min(self.clip_max, self.x_));
            self.y_ = Float::max(self.clip_min, Float::min(self.clip_max, self.y_));
            let mut a = [self.x_, self.y_];

            p.x = T::max(self.clip_min, T::min(self.clip_max, p.x));
            p.y = T::max(self.clip_min, T::min(self.clip_max, p.y));
            let mut b = [p.x, p.y];
            if clip_line(&mut a, &mut b, self.x0, self.y0, self.x1, self.y1) {
                if !self.v_ {
                    if self.use_buffer_stream {
                        self.buffer_stream.line_start();
                        self.buffer_stream
                            .point(&Coordinate { x: a[0], y: a[1] }, None);
                    } else {
                        self.state.sink.line_start();
                        self.state
                            .sink
                            .point(&Coordinate { x: a[0], y: a[1] }, None);
                    }
                }
                if self.use_buffer_stream {
                    self.buffer_stream
                        .point(&Coordinate { x: b[0], y: b[1] }, None);
                } else {
                    self.state
                        .sink
                        .point(&Coordinate { x: b[0], y: b[1] }, None);
                }
                if !v {
                    if self.use_buffer_stream {
                        self.buffer_stream.line_end();
                    } else {
                        self.state.sink.line_end();
                    }
                    self.clean = false;
                }
            } else if v {
                if self.use_buffer_stream {
                    self.buffer_stream.line_start();
                    self.buffer_stream.point(&p, None);
                } else {
                    self.state.sink.line_start();
                    self.state.sink.point(&p, None);
                }
                self.clean = false;
            }
        }

        self.x_ = p.x;
        self.y_ = p.y;
        self.v_ = v;
    }

    // fn gen_interpolate(
    //     &self,
    // ) -> Rc<dyn Fn(Option<Coordinate<T>>, Option<Coordinate<T>>, T, &mut SINK)> {
    //     // Is capturing here a good thing.
    //     let x0 = self.x0;
    //     let y0 = self.y0;
    //     let x1 = self.x1;
    //     let y1 = self.y1;

    //     let compare_point = self.gen_compare_point();
    //     let corner = self.gen_corner();
    //     Rc::new(
    //         move |from: Option<Coordinate<T>>,
    //               to: Option<Coordinate<T>>,
    //               direction: T,
    //               stream: &mut SINK| {
    //             let mut a;
    //             let a1;
    //             let direction_i8: i8 = T::to_i8(&direction).unwrap();
    //             match (to, from) {
    //                 (Some(to), Some(from)) => {
    //                     a = corner(&from, &direction);
    //                     a1 = corner(&to, &direction);
    //                     let cp = compare_point(&from, &to) < Ordering::Less;
    //                     let is_direction = direction > T::zero();
    //                     // logical exor: cp ^^ is_direction
    //                     if a != a1 || (cp && !is_direction) || (!cp && is_direction) {
    //                         loop {
    //                             let p = Coordinate {
    //                                 x: if a == 0 || a == 3 { x0 } else { x1 },
    //                                 y: if a > 1 { y1 } else { y0 },
    //                             };
    //                             stream.point(&p, None);

    //                             a = (a + direction_i8 + 4) % 4;
    //                             if a == a1 {
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                 }
    //                 (Some(to), None) => {
    //                     stream.point(&to, None);
    //                 }
    //                 _ => {
    //                     panic!("did not expect only from and no to .. or Nothing at all Does the JS version get here?");
    //                 }
    //             }
    //         },
    //     )
    // }

    // #[inline]
    // fn compare_intersection(
    //     &self,
    //     a: &Rc<RefCell<Intersection<T>>>,
    //     b: &Rc<RefCell<Intersection<T>>>,
    // ) -> Ordering {
    //     self.compare_point(&a.borrow().x.p, &b.borrow().x.p)
    // }
}

impl<EP, SC, T> Connectable for Rectangle<EP, SC, Unconnected, T>
where
    T: CoordFloat + FloatConst,
{
    type Output = Rectangle<EP, SC, Connected<SC>, T>;
    type SC = SC;
    fn connect(self, sink: SC) -> Self::Output {
        Rectangle {
            state: Connected { sink },
            p_ep: self.p_ep,
            p_sink: self.p_sink,
            buffer_stream: self.buffer_stream,
            clean: self.clean,
            clip_min: self.clip_min,
            clip_max: self.clip_max,
            first: self.first,

            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,

            polygon: self.polygon,
            segments: self.segments,

            // first point.
            x__: self.x__,
            y__: self.y__,
            v__: self.v__,
            // previous point.
            x_: self.x_,
            y_: self.y_,
            v_: self.v__,

            use_line_point: self.use_line_point,
            use_buffer_stream: self.use_buffer_stream,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<EP, SINK, T> Stream for Rectangle<EP, SINK, Connected<SINK>, T>
where
    EP: 'static + Stream<EP = EP, T = T> + Default,
    SINK: 'static + Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.get_endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.use_line_point {
            self.line_point(p, m);
        } else {
            self.default_point(p, m);
        }
    }

    // Buffer geometry within a polygon and then clip it en masse.
    fn polygon_start(&mut self) {
        self.use_buffer_stream = true;

        self.segments = Some(VecDeque::new());
        self.polygon = Some(Vec::new());
        self.clean = true;
    }

    fn polygon_end(&mut self) {
        let start_inside = self.polygon_inside();
        let clean_inside = self.clean && start_inside;
        // Performance if all lengths are known. Can I flatern into a
        // array of arrays or something that implies a contigious block of memory.
        let merged_segments = self
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
                self.state.sink.polygon_start();
            }

            // TODO: Can I haz a single interpolator?
            let interpolator = Interpolator::new(self.x0, self.y0, self.x1, self.y1);
            let interpolator_rejoin = Interpolator::new(self.x0, self.y0, self.x1, self.y1);
            if clean_inside {
                self.state.sink.line_start();
                interpolator.interpolate(None, None, T::one(), &mut self.state.sink);
                self.state.sink.line_end();
            }

            let compare_intersection: CompareIntersectionsFn<T> = Box::new(
                move |a: &Rc<RefCell<Intersection<T>>>,
                      b: &Rc<RefCell<Intersection<T>>>|
                      -> Ordering {
                    interpolator.compare_point(&a.borrow().x.p, &b.borrow().x.p)
                },
            );

            if visible {
                clip_rejoin(
                    &merged_segments,
                    compare_intersection,
                    start_inside,
                    &interpolator_rejoin,
                    &mut self.state.sink,
                );
            }

            self.state.sink.polygon_end();
        }
    }

    fn line_start(&mut self) {
        self.use_line_point = true;
        if let Some(polygon) = &mut self.polygon {
            polygon.push(Vec::new());
        }
        self.first = true;
        self.v_ = false;
        self.x_ = T::nan();
        self.y_ = T::nan();
    }

    fn line_end(&mut self) {
        if self.segments.is_some() {
            self.line_point(
                &Coordinate {
                    x: self.x__,
                    y: self.y__,
                },
                None,
            );
            if self.v__ && self.v_ {
                self.buffer_stream.rejoin();
            }

            self.segments
                .as_mut()
                .unwrap()
                .push_back(self.buffer_stream.result());
        }
        self.use_line_point = false;
        if self.v_ {
            if self.use_buffer_stream {
                self.buffer_stream.line_end();
            } else {
                self.state.sink.line_end();
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Interpolator<T> {
    x0: T,
    y0: T,
    x1: T,
    y1: T,
    epsilon: T,
}

impl<T> Interpolator<T>
where
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    fn new(x0: T, y0: T, x1: T, y1: T) -> Self {
        Self {
            x0,
            y0,
            x1,
            y1,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }

    #[inline]
    fn corner(&self, p: &Coordinate<T>, direction: &T) -> i8 {
        if (p.x - self.x0).abs() < self.epsilon {
            if direction > &T::zero() {
                0
            } else {
                3
            }
        } else if (p.x - self.x1).abs() < self.epsilon {
            if direction > &T::zero() {
                2
            } else {
                1
            }
        } else if (p.y - self.y0).abs() < self.epsilon {
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
    }

    // Warning from JS a, b are LineElem.
    fn compare_point(&self, a: &Coordinate<T>, b: &Coordinate<T>) -> Ordering {
        let ca = self.corner(a, &T::one());
        let cb = self.corner(b, &T::one());
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
    }
}

impl<T> InterpolatorTrait for Interpolator<T>
where
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn interpolate<EP, STREAM>(
        &self,
        to: Option<Coordinate<Self::T>>,
        from: Option<Coordinate<Self::T>>,
        direction: Self::T,
        stream: &mut STREAM,
    ) where
        STREAM: Stream<EP = EP, T = T>,
    {
        // let a = 0;
        // let a1 = 0;
        // let corner = gen_corner();
        let direction_i8: i8 = T::to_i8(&direction).unwrap();
        match (to, from) {
            (Some(to), Some(from)) => {
                let mut a = self.corner(&from, &direction);
                let a1 = self.corner(&to, &direction);
                let cp = self.compare_point(&from, &to) < Ordering::Less;
                let is_direction = direction > T::zero();
                // logical exor: cp ^^ is_direction
                if a != a1 || (cp && !is_direction) || (!cp && is_direction) {
                    loop {
                        let p = Coordinate {
                            x: if a == 0 || a == 3 { self.x0 } else { self.x1 },
                            y: if a > 1 { self.y1 } else { self.y0 },
                        };
                        stream.point(&p, None);

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
    }
}
