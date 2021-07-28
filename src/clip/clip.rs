// use std::collections::VecDeque;
// use std::fmt::Display;
// use std::ops::AddAssign;

// use derivative::Derivative;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::path::PathResultEnum;
// use crate::polygon_contains::contains;
// use crate::projection::projection_trait::ProjectionTrait;
// use crate::stream::stream_dst::StreamDst;
// use crate::stream::Clean;
// use crate::stream::CleanEnum;
// use crate::stream::Stream;
// use crate::Transform;

// use super::antimeridian::line::Line as AntimeridianLine;
// use super::circle::line::Line as CircleLine;
// // use super::clip_base::ClipBase;
// use super::clip_buffer::ClipBuffer;
// // use super::clip_raw::ClipRaw;
// use super::clip_simodnk_enum::ClipSinkEnum;
// use super::interpolate_trait::Interpolate;
// use super::line_elem::LineElem;
// use super::line_enum::LineEnum;
// use super::line_sink_enum::LineSinkEnum;
// use super::point_visible_trait::PointVisible;
// use super::rejoin::rejoin;
// use super::Clip;
// use crate::projection::ProjectionRawTrait;

// #[derive(Derivative)]
// #[derivative(Debug)]
// pub struct Clip<CL, I, PR, PV, T>
// where
//     CL: ClipLineTrait<PR, T>,
//     I: Interpolate<T>,
//     PR: ProjectionRawTrait,
//     PV: PointVisible,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     polygon_started: bool,
//     polygon: Vec<Vec<LineElem<T>>>,
//     ring: Vec<LineElem<T>>,
//     ring_sink: CL,
//     segments: VecDeque<Vec<Vec<LineElem<T>>>>,
//     start: LineElem<T>,
//     sink: ClipSinkEnum<PR, T>,
//     #[derivative(Debug = "ignore")]
//     point_fn: fn(&mut Self, p: &Coordinate<T>, m: Option<u8>),
//     #[derivative(Debug = "ignore")]
//     line_start_fn: fn(&mut Self),
//     #[derivative(Debug = "ignore")]
//     line_end_fn: fn(&mut Self),
// }

// impl<CL, I, PR, PV, T> Clip<CL, I, PR, PV, T>
// where
//     CL: ClipLineTrait<PR, T>,
//     I: Interpolate<T>,
//     PR: ProjectionRawTrait,
//     PV: PointVisible,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     pub fn new(
//         projection_raw: PR,
//         // point_visible: PV,
//         // clip_line: CL,
//         // interpolate: I,
//         start: LineElem<T>,
//     ) -> Self {
//         let ring_buffer = LineSinkEnum::CB(ClipBuffer::default());
//         let mut ring_sink = Self::clip_line.default();
//         ring_sink.stream_in(ring_buffer);
//         let mut segments = VecDeque::new();
//         segments.push_front(vec![vec![]]);
//         Self {
//             // clip_line,
//             // interpolate,
//             // point_visible,
//             segments: segments,
//             polygon_started: false,
//             polygon: vec![vec![]],
//             ring: vec![],
//             start,

//             // raw: ClipRaw::Antimeridian(raw),
//             // base: ClipBase::new(
//             //     projection_raw,
//             //     self.clip_line::default(),
//             //     ring_sink,
//             //     start,
//             // ),
//             point_fn: Self::point_default,
//             line_start_fn: Self::line_start_default,
//             line_end_fn: Self::line_end_default,
//         }
//     }
// }

// impl<CL, I, PR, PV, T> Clip<CL, I, PR, PV, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     // #[inline]
//     // pub fn stream_in(&mut self, stream: ClipSinkEnum<A, T>)
//     // where
//     //     T: CoordFloat + FloatConst,
//     // {
//     //     self.sink = stream;
//     //     match &mut self.line {
//     //         LineEnum::Antimeridian(line) => {
//     //             line.stream_in(LineSinkEnum::CSE(self.base.sink));
//     //         }
//     //         LineEnum::Circle(line) => {
//     //             line.stream_in(LineSinkEnum::CSE(self.base.sink));
//     //         }
//     //         LineEnum::Blank => {
//     //             panic!("Clip stream_in Should not be injecting stream into a  blank.");
//     //         }
//     //     }
//     // }

//     #[inline]
//     fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_default");
//         if Self::point_visible(p, None) {
//             self.sink.point(p, m);
//         }
//     }

//     #[inline]
//     fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_line");
//         self.line.point(p, m);
//     }

//     #[inline]
//     fn line_start_default(&mut self) {
//         println!("clip line_start_default");
//         self.point_fn = Self::point_line;
//         Self::line.line_start();
//     }

//     #[inline]
//     fn line_end_default(&mut self) {
//         println!("clip line_end_default");
//         self.point_fn = Self::point_default;
//         self.base.line.line_end();
//     }

//     #[inline]
//     fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         println!("clip point_ring {:?} {:?}", p, m);
//         // println!("about to ring/push - ring_sink ");
//         // println!("self.base {:#?} ", self.base.ring_sink);
//         self.base.ring.push(LineElem { p: *p, m });
//         self.base.ring_sink.point(p, m);
//         println!("clip point_ring -- end");
//     }

//     #[inline]
//     fn ring_start(&mut self) {
//         println!("clip ring_start");
//         self.base.ring_sink.line_start();
//         self.base.ring.clear();
//         println!("end clip ring_start");
//     }

//     fn ring_end(&mut self) {
//         println!("clip ring_end  entry {:#?}", self.base.ring);
//         let le = self.base.ring[0];
//         // javascript drops m here.
//         self.point_ring(&le.p, None);
//         self.base.ring_sink.line_end();

//         let clean = self.base.ring_sink.clean();
//         // deviation from javascript.
//         // access to the javascript varible 'ring_buffer' is
//         // through the ring_sink varible.
//         // println!("ring_sink {:#?}", self.base.ring_sink);
//         let mut ring_segments = match self.base.ring_sink.get_stream().result() {
//             Some(PathResultEnum::ClipBufferOutput(result)) => {
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

//         self.base.ring.pop();
//         self.base.polygon.push(self.base.ring.clone());
//         // in this javascript version this value is set to NULL
//         // is my assumption that this is valid true?
//         // self.ring = None;
//         self.base.ring = Vec::new();

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
//                     if !self.base.polygon_started {
//                         self.base.sink.polygon_start();
//                         self.base.polygon_started = true;
//                     }
//                     self.base.sink.line_start();
//                     for i in 0..m {
//                         point = segment[i].p;
//                         self.base.sink.point(&point, None);
//                     }
//                     self.base.sink.line_end();
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
//         self.segments.push_back(filtered);
//     }
// }

// impl<CL, I, PR, PV, T> Stream for Clip<CL, I, PR, PV, T>
// where
//     CL: ClipLineTrait<PR, T>,
//     I: Interpolate<T>,
//     PR: ProjectionRawTrait,
//     PV: PointVisible,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     type C = Coordinate<T>;
//     fn get_dst(&self) -> StreamDst<T> {
//         self.sink.get_dst()
//     }

//     #[inline]
//     fn point(&mut self, p: &Self::C, m: Option<u8>) {
//         (self.point_fn)(self, p, m);
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         (self.line_start_fn)(self);
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         (self.line_end_fn)(self);
//     }

//     fn polygon_start(&mut self) {
//         println!("clip  polygon start");
//         self.point_fn = Self::point_ring;
//         self.line_start_fn = Self::ring_start;
//         self.line_end_fn = Self::ring_end;
//         self.segments.clear();
//         self.polygon.clear();
//     }
//     fn polygon_end(&mut self) {
//         println!("clip polygon_end");
//         self.point_fn = Self::point_default;
//         self.line_start_fn = Self::line_start_default;
//         self.line_end_fn = Self::line_end_default;
//         println!("about to merge {:#?}", self.segments);
//         let segments_merged: Vec<Vec<LineElem<T>>> =
//             self.segments.clone().into_iter().flatten().collect();
//         let start_inside = contains(&self.polygon, &self.start);

//         if !segments_merged.is_empty() {
//             println!("mergeed is not empty {:#?}", self.segments);
//             // panic!("pause here");
//             if !self.polygon_started {
//                 self.sink.polygon_start();
//                 self.polygon_started = true;
//             }
//             println!("into rejoin this path");
//             rejoin(&segments_merged, self.raw, start_inside, &mut self.sink);
//         } else if start_inside {
//             if !self.polygon_started {
//                 self.sink.polygon_start();
//                 self.polygon_started = true;
//             }
//             self.sink.line_start();
//             self.interpolate(None, None, T::one(), &mut self.sink);
//             self.sink.line_end();
//         };
//         if self.polygon_started {
//             self.sink.polygon_end();
//             self.polygon_started = false;
//         }
//         self.segments.clear();
//         self.polygon.clear();
//         println!("clip polygon_end -- exit");
//     }

//     fn sphere(&mut self) {
//         self.sink.polygon_start();
//         self.sink.line_start();
//         self.interpolate(None, None, T::one(), &mut self.sink);
//         self.sink.line_end();
//         self.sink.polygon_end();
//     }
// }
