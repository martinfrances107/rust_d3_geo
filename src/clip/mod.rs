use num_traits::Float;
use num_traits::FloatConst;
use num_triats::FloatConst::FRAC_PI_2;
use  crate::math::EPSILON;
use  crate::math::HALFPI;

use crate::projection::geo_stream::GeoStream;

pub mod antimeridian;

mod antimeridian_interpolate;
mod antimeridian_intersect;

// import clipBuffer from "./buffer.js";
// import clipRejoin from "./rejoin.js";
// import {epsilon, halfPI} from "../math.js";
// import polygonContains from "../polygonContains.js";
// import {merge} from "d3-array";

// export default function(pointVisible, clipLine, interpolate, start) {

// alias ClipLineFn = Box<dyn Fn(sink: GeoStream)>;
// alias ClipBufferFn = Box<dyn Fn(sink: GeoStream)>;
type InterpolateFn = Box<dyn Fn(Option<f64>, Option<f64>, f64, dyn GeoStream)>;
type PointVisibleFn = Box<dyn Fn(f64, f64) -> bool>;
type ClipLineFn = Box<dyn Fn(dyn GeoStream)>;

struct Clip<T> {
  clip_line: ClipLineFn,
  interpolate: InterpolateFn,
  start: Box<dyn Fn()>,
  line: Box<dyn Fn(dyn GeoStream)>,
  ring_buffer: Vec<[T;2]>,
  ring_sink: Box<dyn GeoStream>,
  polygon_started:bool,

  point: Box<dyn Fn()>,
  point_visible: PointVisibleFn,
  polygon: Vec<[T;2]>,
  segments: Vec<[T;2]>,
  ring: Vec<[T;2]>,
}

// impl Clip {
//   fn new(point_visible: PointVisibleFn, clip_line: ClipLineFn, interpolate: InterpolateFn, start:f64) -> Self {
//     return Self{
//       interpolate,
//       clip_line,
//       // line: clipLine(sink:GeoStream),
//       point_visible,
//       polygon: None,
//       // polygon_started:false,
//       ring: Vec::new(),
//       ring_buffer: clipBuffer(),
//       ring_sink: clip_line(ringBuffer),
//       segments: None,
//     };
//    }

//    fn validSegment(segment: &Vec<[f64;2]>) -> bool {
//     return segment.len() > 1;
//   }
// }

// impl GeoStream for Clip {

//      fn point(&mut self, lambda:f64, phi:f64) {
//        self.point(lambda, phi);
//      }

//      fn line_start(&mut self) {
//       self.line_start();
//      }

//     fn line_end(&mut self) {
//       self.line_end();
//     }

//     fn polygon_start(&mut self) {
//       // self.point = pointRing;
//       // self.lineStart = ringStart;
//       // self.lineEnd = ringEnd;
//       self.segments.clear();
//       self.polygon.clear();
//     }

//     fn polygon_end(&mut self) {
//         // point = point;
//         // clip.lineStart = lineStart;
//         // clip.lineEnd = lineEnd;
//         // segments = merge(segments);
//         let  startInside = polygon_contains(self.polygon, self.start);
//         if self.segments.len() > 0 {
//           if (!polygonStarted) self.sink.polygonStart(), self.polygonStarted = true;
//           clipRejoin(segments, compareIntersection, startInside, interpolate, sink);
//         } else if (start_inside) {
//           if (!polygon_started) {
//             self.sink.polygonStart();
//             self.polygon_started = true;
//           }
//           sink.lineStart();
//           self.interpolate(None, None, 1, sink);
//           sink.lineEnd();
//         }
//         if (polygonStarted) sink.polygonEnd(), polygonStarted = false;
//         segments = polygon = None;
//       }

//     fn sphere(&mut self) {
//         sink.polygonStart();
//         sink.lineStart();
//         self.interpolate(None, None, 1, sink);
//         sink.lineEnd();
//         sink.polygonEnd();
//       }
//     };

//     fn point(&mut self, lambda:f64, phi:f64) {
//       if (pointVisible(lambda, phi)) self.sink.point(lambda, phi);
//     }

//     fn point_line(&mut self,lambda: f64, phi:f64) {
//       self.line.point(lambda, phi);
//     }

//     fn line_start(&mut self) {
//       self.clip.point = pointLine;
//       self.line.lineStart();
//     }

//     fn line_end(&mut self) {
//       self.clip.point = point;
//       self.line.lineEnd();
//     }

//     // fn pointRing(&self,lambda: f64, phi: f64) {
//     //   self.ring.push([lambda, phi]);
//     //   self.ringSink.point(lambda, phi);
//     // }

//     fn ring_start(&self) {
//       self.ringSink.lineStart();
//       self.ring = [];
//     }

//     fn ring_end(&self) {
//       pointRing(self.ring[0][0], self.ring[0][1]);
//       self.ringSink.lineEnd();

//       let clean = self.ringSink.clean();
//       let ringSegments = ringBuffer.result(),
//       let i;
//       let n = ringSegments.length;
//       let m,
//       let segment,
//       let point;

//       self.ring.pop();
//       self.polygon.push(ring);
//       self.ring = None;

//       if (!n) return;

//       // No intersections.
//       if (self.clean & 1) {
//         self.segment = ringSegments[0];
//         if ((m = segment.length - 1) > 0) {
//           if (!polygonStarted) sink.polygonStart(), polygonStarted = true;
//           sink.lineStart();
//           for (i = 0; i < m; ++i) sink.point((point = segment[i])[0], point[1]);
//           sink.lineEnd();
//         }
//         return;
//       }

//       // Rejoin connected segments.
//       // TODO reuse ringBuffer.rejoin()?
//       if (n > 1 && clean & 2) self.ringSegments.push(self.ringSegments.pop().concat(self.ringSegments.shift()));

//       segments.push(ringSegments.filter(validSegment));

//     return clip;
//   }




/// Intersections are sorted along the clip edge. For both antimeridian cutting
/// and circle clipPIng, the same comparison is used.
// fn compareIntersection<T>(a: [T;2], b:[T;2]) {
//   return (if (a = a.x)[0] < T::zero() { a[1] - FRAC_PI_2 - EPSILON } : {FRAC_PI_2 - a[1]})
//        - (if (b = b.x)[0] < T::zero() { b[1] - FRAC_PI_2 - EPSILON } : {FRAC_PI_2 - b[1]});
// }
