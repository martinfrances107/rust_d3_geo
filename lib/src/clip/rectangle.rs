use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::clip::interpolator::Interpolator;
use crate::path::Result;
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

///A primitive type used for a `PostClipNode` pipeline stage.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
pub struct Rectangle<STATE, T>
where
    STATE: Clone,
    T: CoordFloat,
{
    state: STATE,
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
}

impl<T> Rectangle<Unconnected, T>
where
    T: CoordFloat,
{
    #[inline]
    pub(crate) fn new(x0: T, y0: T, x1: T, y1: T) -> Self {
        Self {
            state: Unconnected,
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
        }
    }
}

impl<SINK, T> Rectangle<Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    fn polygon_inside(&self) -> bool {
        let mut winding = 0;

        if let Some(polygon) = &self.polygon {
            for p in polygon.iter() {
                let mut point = p[0];
                let mut b0 = point.x;
                let mut b1 = point.y;
                for ring in p.iter().skip(1) {
                    let a0 = b0;
                    let a1 = b1;
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
                        winding -= 1;
                    }
                }
            }
        }
        !winding.is_zero()
    }

    #[inline]
    fn visible(&self, p: &Coordinate<T>) -> bool {
        self.x0 <= p.x && p.x <= self.x1 && self.y0 <= p.y && p.y <= self.y1
    }
}

impl<EP, SINK, T> Rectangle<Connected<SINK>, T>
where
    SINK: Clone + Stream<EP = EP, T = T>,
    T: CoordFloat,
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
            self.x_ = T::max(self.clip_min, T::min(self.clip_max, self.x_));
            self.y_ = T::max(self.clip_min, T::min(self.clip_max, self.y_));
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
}

impl<T> Connectable for Rectangle<Unconnected, T>
where
    T: CoordFloat,
{
    /// The resultant postclip node  type.
    type Output<SC: Clone> = Rectangle<Connected<SC>, T>;

    fn connect<SC: Clone>(self, sink: SC) -> Self::Output<SC> {
        Rectangle {
            state: Connected { sink },
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
        }
    }
}

impl<EP, SINK, T> Stream for Rectangle<Connected<SINK>, T>
where
    SINK: Clone + Stream<EP = EP, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
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

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        if self.use_line_point {
            self.line_point(p, m);
        } else {
            self.default_point(p, m);
        }
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

    // Buffer geometry within a polygon and then clip it en masse.
    fn polygon_start(&mut self) {
        self.use_buffer_stream = true;

        self.segments = Some(VecDeque::new());
        self.polygon = Some(Vec::new());
        self.clean = true;
    }
}
