mod intersect;
pub mod line;
mod rejoin;

use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

// use derivative::Derivative;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::circle::circle_stream::circle_stream;
use crate::clip::interpolate_trait::Interpolate;
use crate::clip::point_visible_trait::PointVisible;
use crate::path::PathResult;
// use crate::polygon_contains::contains;
// use crate::projection::ProjectionRawTrait;
// use crate::stream::CompareIntersection;
// use crate::clip::line_sink_enum::LineSinkEnum;
use crate::path::PathResultEnum;
// use crate::stream::stream_dst::StreamDst;
// use crate::clip::Clean;
// use crate::clip::clean::CleanEnum;
// use crate::clip::rejoin::Rejoin;
use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;
use crate::Transform;
// use super::clip::Clip;
// use super::clip_raw::ClipRaw;
use super::clip_base::ClipBase;
// use super::compare_intersections::compare_intersections;
use super::line_elem::LineElem;
use super::Clip;
use super::ClipBaseState;
use super::ClipBuffer;
use super::LCB;
use crate::clip::clean::CleanEnum;
// use crate::clip::clip_sink_enum::ClipSinkEnum;
use line::Line;

// #[derive(Derivative)]
// #[derivative(Debug)]
pub struct ClipCircle<SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Stream<SC = Coordinate<T>> + Default,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst + Display,
{
    // // #[derivative(Debug = "ignore")]
    // point_fn: fn(&mut Self, p: &Coordinate<T>, m: Option<u8>),
    // // #[derivative(Debug = "ignore")]
    // line_start_fn: fn(&mut Self),
    // // #[derivative(Debug = "ignore")]
    // line_end_fn: fn(&mut Self),
    pub radius: T,
    small_radius: bool,
    delta: T,
    cr: T,
    base: ClipBase<Line<SINK, T>, SINK, T>,
    // line: Line<PR, T>,
}

impl<SINK, T> ClipBaseState for ClipCircle<SINK, T>
where
    SINK: Default + Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type CBST = T;
    type L = Line<SINK, T>;
    type SINK = SINK;

    fn get_base(self) -> ClipBase<Line<SINK, T>, SINK, T> {
        self.base
    }
    fn set_polygon_started(&mut self, started: bool) {}
    // fn set_point_fn(&mut self, f: fn(&mut Self, p: &Coordinate<Self::CBOT>, m: Option<u8>)) {}
    // fn set_line_start_fn(f: fn(&mut Self)) {}
    // fn set_line_end_fn(f: fn(&mut Self)) {}
    #[inline]
    fn polygon_clear(&mut self) {
        self.base.polygon.clear();
    }

    #[inline]
    fn polygon_push(&mut self, v: Vec<LineElem<Self::CBST>>) {
        self.base.polygon.push(v)
    }

    #[inline]
    fn ring_clear(&mut self) {
        self.base.ring.clear();
    }
    #[inline]
    fn ring_push(&mut self, le: LineElem<Self::CBST>) {
        self.base.ring.push(le);
    }
    #[inline]
    fn ring_pop(&mut self) -> Option<LineElem<Self::CBST>> {
        self.base.ring.pop()
    }

    #[inline]
    fn ring_reset(&mut self) {
        self.base.ring = Vec::new();
    }

    #[inline]
    fn ring_sink_clean(&mut self) -> CleanEnum {
        self.base.ring_sink.clean()
    }

    #[inline]
    fn set_use_point_line(&mut self, u: bool) {
        self.base.use_point_line = u;
    }

    #[inline]
    fn set_use_ring_start(&mut self, u: bool) {
        self.base.use_ring_start = u;
    }

    #[inline]
    fn set_use_ring_end(&mut self, u: bool) {
        self.base.use_ring_end = u;
    }

    #[inline]
    fn segments_clear(&mut self) {
        self.base.segments.clear();
    }
}

impl<'a, SINK, T> Clip for ClipCircle<SINK, T>
where
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Default + Stream<SC = Coordinate<T>> + Default,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst + Display + Debug,
{
    // type SINK = SINK;
    // type T = T;
    // type CC = Coordinate<T>;
    // fn get_sink(&mut self) -> &mut SINK {
    //     &mut self.base.sink
    // }
}

/// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
/// ClipCircle!!!!
impl<S, T> StreamIn for ClipCircle<S, T>
where
    S: Default + Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SInput = S;
    #[inline]
    fn stream_in(&mut self, stream: Self::SInput)
    where
        T: CoordFloat + FloatConst,
    {
        let stream = Rc::new(RefCell::new(stream));
        self.base.line.link_to_stream(stream);
    }
}

/// Returns a clip object
impl<SINK, T> ClipCircle<SINK, T>
where
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Stream<SC = Coordinate<T>> + Default,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst + Display + Debug,
{
    pub fn new<PR>(projection_raw: PR, radius: T) -> Self
    where
        // Rc<PR>: Transform<C = Coordinate<T>>,
        PR: Transform<C = Coordinate<T>>,
    {
        let cr = radius.cos();
        let small_radius = cr > T::zero();
        let start;
        if small_radius {
            start = LineElem {
                p: Coordinate {
                    x: T::zero(),
                    y: T::zero() - radius,
                },
                m: None,
            }
        } else {
            start = LineElem {
                p: Coordinate {
                    x: -T::PI(),
                    y: radius - T::PI(),
                },
                m: None,
            }
        }

        // let cr = ClipRaw::Circle(ClipCircle {
        //     radius,
        //     delta: T::from(6u8).unwrap() * radius,
        //     small_radius,
        //     cr,
        // });
        // This breaks DRY construction of the parameters for
        // the ClipBase::new() call is common between ClipCircle and
        // ClipAntimeridian.
        let line = Line::new(radius);
        let ring_buffer = Rc::new(RefCell::new(ClipBuffer::default()));
        let mut ring_sink = Box::new(Line::new(radius));
        ring_sink.link_to_stream(ring_buffer.clone());
        ClipCircle {
            // line: Line::default(),
            delta: T::from(6).unwrap().to_radians(),
            cr,
            radius,
            small_radius,
            // start,
            // line_end_fn: Self::line_end_default,
            // point_fn: Self::point_default,
            // line_start_fn: Self::line_start_default,
            // base: ClipBase::new(projection_raw, line, ring_buffer, ring_sink, start),
            base: ClipBase::new(line, ring_buffer, ring_sink, start),
        }
    }
}

impl<SINK, T> PointVisible for ClipCircle<SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Stream<SC = Coordinate<T>> + Default,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst + Display,
{
    type PVC = Coordinate<T>;
    #[inline]
    fn point_visible(&self, p: &Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }
}

// impl<SINK, T> Interpolate for ClipCircle<SINK, T>
// where
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     // MutStream: Stream<SC = Coordinate<T>>,
//     SINK: Stream<SC = Coordinate<T>>
//     // STREAM: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst +Display,
// {
//     type IC = Coordinate<T>;
//     type IT = T;
//     type IStream = SINK;
//     // type IPR = PR;
//     // type IStream = &'a
//     // type ISD = SD;
//     #[inline]
//     fn interpolate(
//         &self,
//     ) -> Box<dyn Fn(Option<Self::IC>, Option<Self::IC>, Self::IT, &mut Self::IStream) + '_> {
//         Box::new(
//             move |from: Option<Coordinate<T>>,
//                   to: Option<Coordinate<T>>,
//                   direction: T,
//                   stream: &mut Self::IStream| {
//                 // todo!("must fix");
//                 circle_stream(stream, self.radius, self.delta, direction, from, to)
//             },
//         )
//     }
// }

impl<SINK, T> Interpolate for ClipCircle<SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Stream<SC = Coordinate<T>> + Default,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    // <Self as Interpolate>::IT: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
{
    type IC = Coordinate<T>;
    type IT = T;
    // type IStream = SINK;
    // type IPR = PR;
    // type IStream = &'a
    // type ISD = SD;
    // fn get_sink(&mut self) -> &mut SINK {
    //     &mut self.base.sink
    // }

    #[inline]
    fn interpolate(
        &mut self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
        // stream: &mut Self::IStream,
    ) {
        // todo!("must fix");

        circle_stream(
            &mut *self.base.sink.borrow_mut(),
            self.radius,
            self.delta,
            direction,
            from,
            to,
        )
    }
}

// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
// ClipCircle!!!!
impl<SINK, T> ClipCircle<SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Stream<SC = Coordinate<T>> + Default,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("clip point_default");
        if self.point_visible(p, None) {
            // self.base.sink.point(p, m);
        }
    }

    #[inline]
    fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("clip point_line");
        self.base.line.point(p, m);
    }

    #[inline]
    fn line_start_default(&mut self) {
        println!("clip line_start_default");
        // self.point_fn = Self::point_line;
        self.line_start();
    }

    #[inline]
    fn line_end_default(&mut self) {
        println!("clip line_end_default");
        // self.point_fn = Self::point_default;
        self.base.line.line_end();
    }

    #[inline]
    fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("clip point_ring {:?} {:?}", p, m);
        // println!("about to ring/push - ring_sink ");
        // println!("self.base {:#?} ", self.base.ring_sink);
        self.base.ring.push(LineElem { p: *p, m });
        self.base.ring_sink.point(p, m);
        println!("clip point_ring -- end");
    }

    #[inline]
    fn ring_start(&mut self) {
        println!("clip ring_start");
        self.base.ring_sink.line_start();
        self.base.ring.clear();
        println!("end clip ring_start");
    }

    fn ring_end(&mut self) {
        println!("clip ring_end  entry {:#?}", self.base.ring);
        let le = self.base.ring[0];
        // javascript drops m here.
        self.point_ring(&le.p, None);
        self.base.ring_sink.line_end();

        let clean = self.base.ring_sink.clean();

        let mut ring_segments = match self.base.ring_buffer.borrow_mut().result() {
            Some(PathResultEnum::ClipBufferOutput(result)) => {
                // Can I find a way of doing this with the expense of dynamic conversion.
                result
            }
            Some(_) => {
                panic!("None buffer ");
            }
            None => panic!("was expecting something."),
        };

        println!("clip ring_end() - ring segments {:#?}", ring_segments);
        // panic!("ring_end buffer result");
        let n = ring_segments.len();
        let m;
        let mut point: Coordinate<T>;

        self.base.ring.pop();
        self.base.polygon.push(self.base.ring.clone());
        // in this javascript version this value is set to NULL
        // is my assumption that this is valid true?
        // self.ring = None;
        self.base.ring = Vec::new();

        if n == 0 {
            return;
        }
        println!("no intersections n, c {:?} {:?}", n, clean);
        // No intersections.
        match clean {
            CleanEnum::NoIntersections => {
                println!("about to clean good path");
                // panic!("on the good path");
                let segment = ring_segments
                    .pop_front()
                    .expect("We have previously checked that the .len() is >0 ( n ) ");
                m = segment.len() - 1;
                if m > 0 {
                    if !self.base.polygon_started {
                        // self.base.sink.polygon_start();
                        self.base.polygon_started = true;
                    }
                    // self.base.sink.line_start();
                    for i in 0..m {
                        point = segment[i].p;
                        // self.base.sink.point(&point, None);
                    }
                    // self.base.sink.line_end();
                }
                return;
            }
            CleanEnum::IntersectionsRejoin => {
                // Rejoin connected segments.
                // TODO reuse ringBuffer.rejoin()?
                if n > 1 {
                    println!("funny buisness");
                    println!("ring_segemtns before fb {:#?}", ring_segments);
                    let pb = [
                        ring_segments.pop_back().unwrap(),
                        ring_segments.pop_front().unwrap(),
                    ]
                    .concat();
                    ring_segments.push_back(pb);
                }
            }
            CleanEnum::IntersectionsOrEmpty => {
                // No-op
            }
            CleanEnum::Undefined => {
                panic!("must be defined by now.")
            }
        }
        println!("final segments before filter {:#?}", ring_segments);
        // panic!("final segments");
        let filtered: Vec<Vec<LineElem<T>>> = ring_segments
            .into_iter()
            .filter(|segment| segment.len() > 1)
            .collect();
        self.base.segments.push_back(filtered);
    }
}

/// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
/// ClipCircle!!!!
impl<SINK, T> Stream for ClipCircle<SINK, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // MutStream: Stream<SC = Coordinate<T>>,
    SINK: Stream<SC = Coordinate<T>> + Default,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;
    // type ST = T;
    // type SD = SD;

    // fn get_dst(
    //     &self,
    // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
    // {
    //     self.base.sink.get_dst()
    // }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        // (self.point_fn)(self, p, m);
    }

    #[inline]
    fn line_start(&mut self) {
        // (self.line_start_fn)(self);
    }

    #[inline]
    fn line_end(&mut self) {
        // (self.line_end_fn)(self);
    }

    fn polygon_start(&mut self) {
        println!("clip  polygon start");
        // self.point_fn = Self::point_ring;
        // self.line_start_fn = Self::ring_start;
        // self.line_end_fn = Self::ring_end;
        self.base.segments.clear();
        self.base.polygon.clear();
    }
    fn polygon_end(&mut self) {
        println!("clip polygon_end");
        // self.point_fn = Self::point_default;
        // self.line_start_fn = Self::line_start_default;
        // self.line_end_fn = Self::line_end_default;
        println!("about to merge {:#?}", self.base.segments);
        let segments_merged: Vec<Vec<LineElem<T>>> =
            self.base.segments.clone().into_iter().flatten().collect();
        // let start_inside = contains(&self.base.polygon, &self.base.start);
        let start_inside = true;

        if !segments_merged.is_empty() {
            println!("mergeed is not empty {:#?}", self.base.segments);
            // panic!("pause here");
            if !self.base.polygon_started {
                // self.base.sink.polygon_start();
                self.base.polygon_started = true;
            }
            println!("into rejoin this path");
            // self.rejoin(
            //     &segments_merged,
            //     compare_intersections,
            //     start_inside,
            //     // self,
            //     // self.interpolate(),
            //     // &mut self.base.sink,
            // );
        } else if start_inside {
            if !self.base.polygon_started {
                // self.base.sink.polygon_start();
                self.base.polygon_started = true;
            }
            // self.base.sink.line_start();
            self.interpolate(None, None, T::one());
            // self.base.sink.line_end();
        };
        if self.base.polygon_started {
            // self.base.sink.polygon_end();
            self.base.polygon_started = false;
        }
        self.base.segments.clear();
        self.base.polygon.clear();
        println!("clip polygon_end -- exit");
    }

    fn sphere(&mut self) {
        // self.base.sink.polygon_start();
        // self.base.sink.line_start();
        self.interpolate(None, None, T::one());
        // self.base.sink.line_end();
        // self.base.sink.polygon_end();
    }
}
