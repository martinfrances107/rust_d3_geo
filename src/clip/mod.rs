// use std::fmt::Debug;

use geo::{CoordFloat, Coordinate};
// use num_traits::FloatConst;

// /// Public: clip generators used by projection.
pub mod antimeridian;
pub mod circle;

// mod buffer;
// mod rejoin;

// use crate::polygon_contains::contains;
use crate::stream::Stream;

// use buffer::ClipBuffer;
// // use rejoin::rejoin;
pub trait ClipLine<'a, T> {
    fn line_start(&mut self);
    fn point(&mut self, lambda1_p: T, phi1: T, m: Option<u8>);
    fn line_end(&mut self);
    fn clean(&mut self) -> Option<u8>;
    fn stream(&mut self, stream: Box<dyn Stream<T>>);
}

// // CompareIntersections param type
// #[derive(Clone, Debug)]
// pub struct Ci<T: CoordFloat> {
//     x: Point<T>,
// }

pub type InterpolateFn<T> =
    Box<dyn Fn(Option<Coordinate<T>>, Option<Coordinate<T>>, T, Box<dyn Stream<T>>)>;
type PointsVisibleFn<T> = Box<dyn Fn(T, T, Option<u8>) -> bool>;
pub type PointVisibleFnPtr<T> = PointsVisibleFn<T>;
type ClipLineFn<T> = Box<dyn Fn(Box<dyn ClipLine<T>>) -> Box<dyn ClipLine<T>>>;
// // pub type CompareIntersectionFn<T> = Box<dyn Fn(Ci<T>, Ci<T>) -> T>;

pub struct Clip<T: CoordFloat> {
    line: Box<dyn Stream<T>>,
    interpolate: InterpolateFn<T>,
    // point: Box<dyn Fn()>,
    // point: Point,
    polygon_started: bool,
    polygon: Box<Vec<Vec<Coordinate<T>>>>,
    point_visible: PointVisibleFnPtr<T>,
    ring_buffer: Box<dyn Stream<T>>,
    ring_sink: Box<dyn Stream<T>>,
    segments: Box<Vec<Vec<Coordinate<T>>>>,
    start: Coordinate<T>,
    ring: Vec<Coordinate<T>>,
    use_ring: bool,
    sink: Box<dyn Stream<T>>,
}

// impl<'a, T: CoordFloat + FloatConst + 'static> Clip<T> {
//     fn new(
//         point_visible: PointVisibleFnPtr<T>,
//         clip_line_fn_ptr: ClipLineFn<T>,
//         interpolate: InterpolateFn<T>,
//         start: Coordinate<T>,
//     ) -> Box<dyn Fn(Box<dyn Stream<T>>) -> Box<dyn Stream<T>>> {
//         return Box::new(|sink:Box<dyn Stream<T>>| {
//             let clip_line = clip_line_fn_ptr;
//             let line = clip_line(sink);

//             let ring_buffer = ClipBuffer::new();
//             let ring_sink = clip_line(ring_buffer);

//             return Box::new(Self {
//                 use_ring: false,
//                 interpolate,
//                 line,
//                 point_visible,
//                 polygon: Box::new(Vec::new()),
//                 polygon_started: false,
//                 ring: Vec::new(),
//                 ring_buffer,
//                 ring_sink,
//                 segments: Box::new(Vec::new()),
//                 sink,
//                 start: start.clone(),
//             });
//         });
//     }

//     #[inline]
//     fn valid_segment(segment: &[[f64; 2]]) -> bool {
//         return segment.len() > 1;
//     }

//     fn point_ring(&mut self, lambda: T, phi: T, _m: Option<u8>) {
//         self.ring.push(Coordinate { x: lambda, y: phi });
//         let mut ring_sink = self.ring_sink;
//         ring_sink.point(lambda, phi, None);
//     }

//     fn ring_start(&self) {
//         self.ringSink.lineStart();
//         self.ring = Vec::new();
//     }

//     //     fn ring_end(&self) {
//     //       pointRing(self.ring[0][0], self.ring[0][1]);
//     //       self.ringSink.lineEnd();

//     //       let clean = self.ringSink.clean();
//     //       let ringSegments = ringBuffer.result(),
//     //       let i;
//     //       let n = ringSegments.length;
//     //       let m,
//     //       let segment,
//     //       let point;

//     //       self.ring.pop();
//     //       self.polygon.push(ring);
//     //       self.ring = None;

//     //       if (!n) return;

//     //       // No intersections.
//     //       if (self.clean & 1) {
//     //         self.segment = ringSegments[0];
//     //         if ((m = segment.length - 1) > 0) {
//     //           if (!polygonStarted) sink.polygonStart(), polygonStarted = true;
//     //           sink.lineStart();
//     //           for (i = 0; i < m; ++i) sink.point((point = segment[i])[0], point[1]);
//     //           sink.lineEnd();
//     //         }
//     //         return;
//     //       }

//     //       // Rejoin connected segments.
//     //       // TODO reuse ringBuffer.rejoin()?
//     //       if (n > 1 && clean & 2) self.ringSegments.push(self.ringSegments.pop().concat(self.ringSegments.shift()));

//     //       segments.push(ringSegments.filter(validSegment));

//     //     return clip;
//     //   }
// }

// impl<'a, T: CoordFloat + FloatConst> Stream<T> for Clip<T> {
//     fn point(&mut self, lambda: T, phi: T, m: Option<u8>) {
//         match self.use_ring {
//             true => {
//                 self.ring.push(Coordinate { x: lambda, y: phi });
//                 // self.ring_sink.point(lambda, phi, None);
//             }
//             false => {
//                 if (self.point_visible)(lambda, phi, None) {
//                     let mut sink = self.sink;
//                     sink.point(lambda, phi, m);
//                 }
//             }
//         }
//     }

//     fn line_start(&mut self) {
//         // self.clip.point = self.point_line;
//         // self.line.line_start();
//     }

//     fn line_end(&mut self) {
//         // self.line_end();
//     }

//     fn polygon_start(&mut self) {
//         // self.point = self.pointRing;
//         // self.line_start = self.ringStart;
//         // self.line_end = self.ringEnd;
//         self.use_ring = true;
//         self.segments.clear();
//         self.polygon.clear();
//     }

//     fn polygon_end(&mut self) {
//         self.use_ring = false;
//         // point = point;
//         // clip.lineStart = lineStart;
//         // clip.lineEnd = lineEnd;
//         // segments = merge(segments);
//         let start_inside = contains(self.polygon.to_vec(), &self.start);
//         let mut sink = self.sink;
//         if !self.polygon_started {
//             let mut sink = self.sink;
//             sink.polygon_start();
//             self.polygon_started = true;

//         // rejoin(self.segments.to_vec(), Box::new(compare_intersection), start_inside, self.interpolate, self.sink);
//         } else if start_inside {
//             if !self.polygon_started {
//                 sink.polygon_start();
//                 self.polygon_started = true;
//             }
//             sink.line_start();
//             // (self.interpolate)(None, None, 1f64, self.sink);
//             sink.line_end();
//         }
//         if self.polygon_started {
//             sink.polygon_end();
//             self.polygon_started = false;
//         }
//         self.segments.clear();
//         self.polygon.clear();
//     }

//     fn sphere(&mut self) {
//         let mut sink = self.sink;
//         sink.polygon_start();
//         sink.line_start();
//         // (self.interpolate)(None, None, 1f64, self.sink);
//         sink.line_end();
//         sink.polygon_end();
//     }
// }

// /// Intersections are sorted along the clip edge. For both antimeridian cutting
// /// and circle clipPIng, the same comparison is used.
// fn compare_intersection<T: CoordFloat + FloatConst>(a: Ci<T>, b: Ci<T>) -> T {
//     let a_dashed = a.x;
//     let part1 = match a_dashed.x() < T::zero() {
//         true => a_dashed.y() - T::FRAC_PI_2() - T::epsilon(),
//         false => T::FRAC_PI_2() - a_dashed.y(),
//     };
//     let b_dashed = b.x;
//     let part2 = match b_dashed.x() < T::zero() {
//         true => b_dashed.y() - T::FRAC_PI_2() - T::epsilon(),
//         false => T::FRAC_PI_2() - b_dashed.y(),
//     };

//     return part1 - part2;
// }
