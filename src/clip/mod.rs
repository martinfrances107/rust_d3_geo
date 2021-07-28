pub mod antimeridian;
pub mod circle;
mod clean;
pub mod clip;
pub mod clip_base;
pub mod clip_buffer;
pub mod clip_raw;
pub mod compare_intersections;
pub mod interpolate_trait;
pub mod line_elem;
pub mod line_trait;
pub mod point_visible_trait;
pub mod rejoin;

use std::cell::RefCell;
// use std::cmp::Ordering;
// use std::fmt::Debug;
// use std::fmt::Display;
// use std::ops::AddAssign;
use std::rc::Rc;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;
// use num_traits::Float;

// use crate::clip::compare_intersections::compare_intersections;
use crate::clip::rejoin::Rejoin;
// use crate::path::PathResult;
// use crate::path::PathResultEnum;
// use crate::polygon_contains::contains;

use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;

// use crate::clip::rejoin::intersection::Intersection;
// use crate::clip::line_elem::LineElem;
// use crate::stream::Stream;
// use crate::clip::rejoin::link::link;
// use clean::CleanEnum;
use clip_buffer::ClipBuffer;
// use rejoin::intersection::Intersection;
// use crate::projection::ProjectionRawTrait;
// use crate::point_equal::point_equal;
// use clip_base::ClipBase;
use interpolate_trait::Interpolate;
use point_visible_trait::PointVisible;
// pub trait Clip: PointVisible + Interpolate + Stream
use clean::Clean;
// use line_elem::LineElem;

/// This trait is connected with the submodule
/// clip_ops_macro_derive
pub trait ClipOpsMacro
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + Debug + FloatConst,
{
    type COT;

    fn hello_macro(&self) -> u32 {
        42
    }
    fn point_default(&mut self, p: &Self::COT, m: Option<u8>);

    fn point_line(&mut self, p: &Self::COT, m: Option<u8>);
    fn line_start_default(&mut self);
    fn line_end_default(&mut self);
    fn point_ring(&mut self, p: &Self::COT, m: Option<u8>);
    fn ring_start(&mut self);
    fn ring_end(&mut self);
}

pub trait Clip: PointVisible + Interpolate + Rejoin + Stream + StreamIn {}

pub trait LCB: Clean + Stream {
    // type T;
    type STREAM;
    fn link_to_stream(&mut self, stream: Rc<RefCell<Self::STREAM>>);
}

// Impl for dyn Clip<> this is common to ClipAntimeridian and ClipCircle.
// impl<L: LCB, S, T> ClipBaseOps<T>
//     for &mut dyn Clip<
//         CBST = T,
//         L = L,
//         IT = T,
//         IC = Coordinate<T>,
//         PVC = Coordinate<T>,
//         T = T,
//         SINK = S,
//         SInput = S,
//         SC = Coordinate<T>,
//     >
// where
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     // MutStream: Stream<SC = Coordinate<T>>,
//     // C: Clip,
//     // SINK: Stream<SC = Coordinate<T>> + Default,
//     L: LCB<SC = Coordinate<T>>,
//     S: Stream<SC = Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type CBOT = T;
//     type L = L;
//     type SINK = S;
//     #[inline]
//     fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_default");
//         if self.point_visible(p, None) {
//             // self.get_base().sink.borrow_mut().point(p, m);
//         }
//     }

//     #[inline]
//     fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_line");
//         // self.get_base().line.point(p, m);
//     }

//     #[inline]
//     fn line_start_default(&mut self) {
//         println!("clip line_start_default");
//         // let base = self.get_base();
//         // self.point_fn = Self::point_line;
//         self.set_use_point_line(true);
//         self.line_start();
//     }

//     #[inline]
//     fn line_end_default(&mut self) {
//         println!("clip line_end_default");
//         // self.point_fn = Self::point_default;
//         self.set_use_point_line(false);
//         // self.get_base().line.line_end();
//     }

//     #[inline]
//     fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_ring {:?} {:?}", p, m);
//         // println!("about to ring/push - ring_sink ");
//         // println!("self.base {:#?} ", self.base.ring_sink);
//         // let mut base = self.get_base();
//         // base.ring.push(LineElem { p: *p, m });
//         // base.ring_sink.point(p, m);
//         println!("clip point_ring -- end");
//     }

//     #[inline]
//     fn ring_start(&mut self) {
//         println!("clip ring_start");
//         // self.get_base().ring_sink.line_start();
//         // self.base.ring.clear();
//         self.ring_clear();
//         println!("end clip ring_start");
//     }

//     fn ring_end(&mut self) {
//         // let mut base = self.get_base();
//         // println!("clip ring_end  entry {:#?}", base.ring);
//         // let le = base.ring[0];
//         // javascript version drops m here.
//         // self.point_ring(&le.p, None);
//         // let mut base = self.get_base();
//         // base.ring_sink.line_end();

//         // let clean = self.base.ring_sink.clean();
//         let clean = self.ring_sink_clean();

//         // let mut ring_segments = match self.get_base().ring_buffer.borrow_mut().result() {
//         //     Some(PathResultEnum::ClipBufferOutput(result)) => {
//         //         // Can I find a way of doing this with the expense of dynamic conversion.
//         //         result
//         //     }
//         //     Some(_) => {
//         //         panic!("None buffer ");
//         //     }
//         //     None => panic!("was expecting something."),
//         // };

//         // println!("clip ring_end() - ring segments {:#?}", ring_segments);
//         // panic!("ring_end buffer result");
//         // let n = ring_segments.len();
//         // let m;
//         let mut point: Coordinate<T>;

//         self.ring_pop();
//         // self.base.polygon.push(self.base.ring.clone());
//         // self.polygon_push(self.get_base().ring.clone());
//         // in this javascript version this value is set to NULL
//         // is my assumption that this is valid true?
//         // self.ring = None;
//         // self.base.ring = Vec::new();
//         self.ring_reset();

//         // if n == 0 {
//         //     return;
//         // }
//         // println!("no intersections n, c {:?} {:?}", n, clean);
//         // No intersections.
//         // match clean {
//         //     CleanEnum::NoIntersections => {
//         //         println!("about to clean good path");
//         //         // panic!("on the good path");
//         //         // let segment = ring_segments
//         //         //     .pop_front()
//         //         //     .expect("We have previously checked that the .len() is >0 ( n ) ");
//         //         // m = segment.len() - 1;
//         //         if m > 0 {
//         //             let base = self.get_base();
//         //             if !base.polygon_started {
//         //                 base.sink.borrow_mut().polygon_start();
//         //                 // self.base.polygon_started = true;
//         //                 self.set_polygon_started(true);
//         //             }
//         //             self.get_base().sink.borrow_mut().line_start();
//         //             for i in 0..m {
//         //                 // point = segment[i].p;
//         //                 // self.get_base().sink.borrow_mut().point(&point, None);
//         //             }
//         //             // self.get_base().sink.borrow_mut().line_end();
//         //         }
//         //         return;
//         //     }
//         //     // CleanEnum::IntersectionsRejoin => {
//         //     //     // Rejoin connected segments.
//         //     //     // TODO reuse ringBuffer.rejoin()?
//         //     //     if n > 1 {
//         //     //         println!("funny buisness");
//         //     //         println!("ring_segemtns before fb {:#?}", ring_segments);
//         //     //         let pb = [
//         //     //             ring_segments.pop_back().unwrap(),
//         //     //             ring_segments.pop_front().unwrap(),
//         //     //         ]
//         //     //         .concat();
//         //     //         ring_segments.push_back(pb);
//         //     //     }
//         //     // }
//         //     CleanEnum::IntersectionsOrEmpty => {
//         //         // No-op
//         //     }
//         //     CleanEnum::Undefined => {
//         //         panic!("must be defined by now.")
//         //     }
//         // }
//         // println!("final segments before filter {:#?}", ring_segments);
//         // panic!("final segments");
//         // let filtered: Vec<Vec<LineElem<T>>> = ring_segments
//         //     .into_iter()
//         //     .filter(|segment| segment.len() > 1)
//         //     .collect();
//         // self.get_base().segments.push_back(filtered);
//     }
// }

// Warning this breaks DRY, the stream is common to both ClipAntimeridian and
// ClipCircle!!!!
// impl<L, SINK, SInput, T> Stream
//     for dyn Clip<
//         CBST = T,
//         L = L,
//         PVC = Coordinate<T>,
//         IT = T,
//         IC = Coordinate<T>,
//         SC = Coordinate<T>,
//         T = T,
//         SINK = SINK,
//         SInput = SInput,
//     >
// where
//     // C: Clip<CBST = T, L = L, PVC = Coordinate<T>, IT = T, IC = Coordinate<T>, T = T>,
//     C: ClipBaseOps<T>,
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     // MutStream: Stream<SC = Coordinate<T>>,
//     // SINK: Stream<SC = Coordinate<T>> + Default,
//     // C: ClipBase<T>,
//     // S: Stream<SC = Coordinate<T>> + Default,
//     L: LCB,
//     L: Stream<SC = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type SC = Coordinate<T>;
//     // type ST = T;
//     // type SD = SD;

//     // fn get_dst(
//     //     &self,
//     // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
//     // {
//     //     self.base.sink.get_dst()
//     // }

//     #[inline]
//     fn point(&mut self, p: &Self::SC, m: Option<u8>) {
//         if self.get_base().use_point_line {
//             self.point_line(p, m);
//         } else {
//             self.point_default(p, m);
//         }
//         // (self.get_base().point_fn)(self, p, m);
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         if self.get_base().use_ring_start {
//             self.ring_start();
//         } else {
//             self.line_start_default();
//         }
//         // (self.get_base().line_start_fn)(self);
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         if self.get_base().use_ring_end {
//             self.ring_end();
//         } else {
//             self.line_end_default();
//         }
//         // (self.get_base().line_end_fn)(self);
//     }

//     fn polygon_start(&mut self) {
//         println!("clip  polygon start");
//         // self.set_point_fn(Self::point_ring);
//         // self.line_start_fn = Self::ring_start;
//         // self.line_end_fn = Self::ring_end;
//         self.set_use_point_line(true);
//         self.set_use_ring_start(true);
//         self.set_use_ring_end(true);

//         self.segments_clear();
//         self.polygon_clear();
//         // self.base.segments.clear();
//         // self.base.polygon.clear();
//     }

//     fn polygon_end(&mut self) {
//         println!("clip polygon_end");

//         // self.point_fn = Self::point_default;
//         // self.line_start_fn = Self::line_start_default;
//         // self.line_end_fn = Self::line_end_default;

//         self.set_use_point_line(false);
//         self.set_use_ring_start(false);
//         self.set_use_ring_end(false);

//         let base = self.get_base();
//         println!("about to merge {:#?}", base.segments);
//         let segments_merged: Vec<Vec<LineElem<T>>> =
//             base.segments.clone().into_iter().flatten().collect();
//         let start_inside = contains(&base.polygon, &base.start);

//         if !segments_merged.is_empty() {
//             println!("mergeed is not empty {:#?}", base.segments);
//             // panic!("pause here");
//             if !base.polygon_started {
//                 base.sink.borrow_mut().polygon_start();
//                 self.set_polygon_started(true);
//                 // self.base.polygon_started = true;
//             }
//             println!("into rejoin this path");
//             self.rejoin(
//                 &segments_merged,
//                 compare_intersections,
//                 start_inside,
//                 // self,
//                 // self.interpolate(),
//                 // &mut self.base.sink,
//             );
//         } else if start_inside {
//             if !self.get_base().polygon_started {
//                 self.get_base().sink.borrow_mut().polygon_start();
//                 // self.base.polygon_started = true;
//                 self.set_polygon_started(true);
//             }
//             base.sink.borrow_mut().line_start();
//             self.interpolate(None, None, T::one());
//             base.sink.borrow_mut().line_end();
//         };
//         if self.get_base().polygon_started {
//             self.get_base().sink.borrow_mut().polygon_end();
//             // self.base.polygon_started = false;
//             self.set_polygon_started(false);
//         }
//         self.segments_clear();
//         self.polygon_clear();
//         // self.base.segments.clear();
//         // self.base.polygon.clear();
//         println!("clip polygon_end -- exit");
//     }

//     fn sphere(&mut self) {
//         let base = self.get_base();
//         base.sink.borrow_mut().polygon_start();
//         base.sink.borrow_mut().line_start();
//         self.interpolate(None, None, T::one());
//         base.sink.borrow_mut().line_end();
//         base.sink.borrow_mut().polygon_end();
//     }
// }
