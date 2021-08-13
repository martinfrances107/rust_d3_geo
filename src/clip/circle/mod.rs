pub mod interpolate;
mod intersect;
pub mod line;
pub mod pv;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use std::fmt::Display;
use std::ops::AddAssign;

// use crate::clip::circle::interpolate::Interpolate;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use crate::clip::circle::interpolate::generate as generate_interpolate;

pub(crate) fn gen_clip_factory_circle<PR, SINK, T>(
    projection_raw: PR,
    radius: T,
) -> StreamNodeClipFactory<Line<T>, PR, PV<T>, SINK, T>
where
    PR: ProjectionRaw<T = T>,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    StreamNodeClipFactory::new(
        generate_interpolate(radius),
        Line::new(radius),
        PV { cr: radius.cos() },
    )
}

// use super::LCB;
// use crate::clip::clean::CleanEnum;
// use line::Line;

// #[derive(Derivative)]
// #[derivative(Debug)]
// pub struct ClipCircle<T>
// where
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     // MutStream: Stream<SC = Coordinate<T>>,
//     // SINK: Stream<SC = Coordinate<T>> + Default,
//     // STREAM: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     // // #[derivative(Debug = "ignore")]
//     // point_fn: fn(&mut Self, p: &Coordinate<T>, m: Option<u8>),
//     // // #[derivative(Debug = "ignore")]
//     // line_start_fn: fn(&mut Self),
//     // // #[derivative(Debug = "ignore")]
//     // line_end_fn: fn(&mut Self),
//     pub radius: T,
//     // small_radius: bool,
//     delta: T,
//     cr: T,
//     base: ClipBase<T>,
//     // line: Line<PR, T>,
// }

// use interpolate::Interpolate;
// use pv::PV;
// impl<T> StreamNodeFactory<Clip<PV<T>, Line<T>, Interpolate<T>, T>, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + Debug + FloatConst,
// {
//     pub fn new<PR>(_projection_raw: PR, radius: T) -> Self
//     where
//         PR: Transform<C = Coordinate<T>>,
//     {
//         let cr = radius.cos();
//         let small_radius = cr > T::zero();
//         let start;
//         if small_radius {
//             start = LineElem {
//                 p: Coordinate {
//                     x: T::zero(),
//                     y: T::zero() - radius,
//                 },
//                 m: None,
//             }
//         } else {
//             start = LineElem {
//                 p: Coordinate {
//                     x: -T::PI(),
//                     y: radius - T::PI(),
//                 },
//                 m: None,
//             }
//         }

//         StreamNodeFactory::new(
//             Clip::default(), //     ClipCircle{

//                              //     radius,
//                              //     cr,
//                              //     delta: T::from(6).unwrap().to_radians(),
//                              // })
//         )
//     }
// }

// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
// ClipCircle!!!!
// impl<T> StreamNode<Clip<Interpolate<T>, Line<T>, PV<T>, T>, T>
// where
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     // MutStream: Stream<SC = Coordinate<T>>,
//     // SINK: Stream<SC = Coordinate<T>> + Default,
//     // STREAM: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     #[inline]
//     fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_default");
//         if self.raw.pv.point_visible(p, None) {
//             // self.base.sink.point(p, m);
//         }
//     }

//     #[inline]
//     fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_line");
//         self.raw.line.point(p, m);
//     }

//     #[inline]
//     fn line_start_default(&mut self) {
//         println!("clip line_start_default");
//         // self.base.point_fn = Self::point_line;
//         self.raw.line_start();
//     }

//     #[inline]
//     fn line_end_default(&mut self) {
//         println!("clip line_end_default");
//         // self.point_fn = Self::point_default;
//         self.raw.line.line_end();
//     }

//     #[inline]
//     fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_ring {:?} {:?}", p, m);
//         // println!("about to ring/push - ring_sink ");
//         // println!("self.base {:#?} ", self.base.ring_sink);
//         self.raw.ring.push(LineElem { p: *p, m });
//         self.raw.ring_sink.borrow_mut().point(p, m);
//         println!("clip point_ring -- end");
//     }

//     #[inline]
//     fn ring_start(&mut self) {
//         println!("clip ring_start");
//         self.raw.ring_sink.borrow_mut().line_start();
//         self.raw.ring.clear();
//         println!("end clip ring_start");
//     }

//     fn ring_end(&mut self) {
//         println!("clip ring_end  entry {:#?}", self.raw.ring);
//         let le = self.raw.ring[0];
//         // javascript drops m here.
//         self.point_ring(&le.p, None);
//         self.raw.ring_sink.borrow_mut().line_end();

//         let clean = self.raw.ring_sink.borrow_mut().clean();

//         let mut ring_segments = match self.raw.ring_buffer.borrow_mut().result() {
//             Some(ResultEnum::ClipBufferOutput(result)) => {
//                 // Can I find a way of doing this with the expense of dynamic conversion.
//                 result
//             }
//             Some(_) => {
//                 panic!("None buffer ");
//             }
//             None => panic!("was expecting something."),
//         };

//         println!("clip ring_end() - ring segments {:#?}", ring_segments);
//         // panic!("ring_end buffer result");
//         let n = ring_segments.len();
//         let m;
//         let mut point: Coordinate<T>;

//         self.raw.ring.pop();
//         self.raw.polygon.push(self.raw.ring.clone());
//         // in this javascript version this value is set to NULL
//         // is my assumption that this is valid true?
//         // self.ring = None;
//         self.raw.ring = Vec::new();

//         if n == 0 {
//             return;
//         }
//         println!("no intersections n, c {:?} {:?}", n, clean);
//         // No intersections.
//         match clean {
//             CleanEnum::NoIntersections => {
//                 println!("about to clean good path");
//                 // panic!("on the good path");
//                 let segment = ring_segments
//                     .pop_front()
//                     .expect("We have previously checked that the .len() is >0 ( n ) ");
//                 m = segment.len() - 1;
//                 if m > 0 {
//                     if !self.raw.polygon_started {
//                         // self.base.sink.polygon_start();
//                         self.raw.polygon_started = true;
//                     }
//                     // self.base.sink.line_start();
//                     for i in 0..m {
//                         point = segment[i].p;
//                         // self.base.sink.point(&point, None);
//                     }
//                     // self.base.sink.line_end();
//                 }
//                 return;
//             }
//             CleanEnum::IntersectionsRejoin => {
//                 // Rejoin connected segments.
//                 // TODO reuse ringBuffer.rejoin()?
//                 if n > 1 {
//                     println!("funny buisness");
//                     println!("ring_segemtns before fb {:#?}", ring_segments);
//                     let pb = [
//                         ring_segments.pop_back().unwrap(),
//                         ring_segments.pop_front().unwrap(),
//                     ]
//                     .concat();
//                     ring_segments.push_back(pb);
//                 }
//             }
//             CleanEnum::IntersectionsOrEmpty => {
//                 // No-op
//             }
//             CleanEnum::Undefined => {
//                 panic!("must be defined by now.")
//             }
//         }
//         println!("final segments before filter {:#?}", ring_segments);
//         // panic!("final segments");
//         let filtered: Vec<Vec<LineElem<T>>> = ring_segments
//             .into_iter()
//             .filter(|segment| segment.len() > 1)
//             .collect();
//         self.raw.segments.push_back(filtered);
//     }
// }

// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
// ClipCircle!!!!
// impl<T> Stream for StreamNode<Clip<Interpolate<T>, Line<T>, PV<T>, T>, T>
// where
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     // MutStream: Stream<SC = Coordinate<T>>,
//     // SINK: Stream<SC = Coordinate<T>> + Default,
//     // STREAM: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type SC = Coordinate<T>;

//     #[inline]
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         // (self.point_fn)(self, p, m);
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         // (self.line_start_fn)(self);
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         // (self.line_end_fn)(self);
//     }

//     fn polygon_start(&mut self) {
//         println!("clip  polygon start");
//         // self.point_fn = Self::point_ring;
//         // self.line_start_fn = Self::ring_start;
//         // self.line_end_fn = Self::ring_end;
//         self.raw.segments.clear();
//         self.raw.polygon.clear();
//     }
//     fn polygon_end(&mut self) {
//         println!("clip polygon_end");
//         // self.point_fn = Self::point_default;
//         // self.line_start_fn = Self::line_start_default;
//         // self.line_end_fn = Self::line_end_default;
//         println!("about to merge {:#?}", self.raw.segments);
//         let segments_merged: Vec<Vec<LineElem<T>>> =
//             self.raw.segments.clone().into_iter().flatten().collect();
//         // let start_inside = contains(&self.base.polygon, &self.base.start);
//         let start_inside = true;

//         if !segments_merged.is_empty() {
//             println!("mergeed is not empty {:#?}", self.raw.segments);
//             // panic!("pause here");
//             if !self.raw.polygon_started {
//                 // self.base.sink.polygon_start();
//                 self.raw.polygon_started = true;
//             }
//             println!("into rejoin this path");
//             // self.rejoin(
//             //     &segments_merged,
//             //     compare_intersections,
//             //     start_inside,
//             //     // self,
//             //     // self.interpolate(),
//             //     // &mut self.base.sink,
//             // );
//         } else if start_inside {
//             if !self.raw.base.polygon_started {
//                 // self.base.sink.polygon_start();
//                 self.raw.base.polygon_started = true;
//             }
//             // self.base.sink.line_start();
//             self.interpolate(None, None, T::one());
//             // self.base.sink.line_end();
//         };
//         if self.raw.base.polygon_started {
//             // self.base.sink.polygon_end();
//             self.raw.base.polygon_started = false;
//         }
//         self.raw.base.segments.clear();
//         self.raw.base.polygon.clear();
//         println!("clip polygon_end -- exit");
//     }

//     fn sphere(&mut self) {
//         // self.base.sink.polygon_start();
//         // self.base.sink.line_start();
//         self.interpolate(None, None, T::one());
//         // self.base.sink.line_end();
//         // self.base.sink.polygon_end();
//     }
// }
