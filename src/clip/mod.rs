use num_traits::Float;
use num_traits::FloatConst;

pub mod antimeridian;
mod rejoin;
mod buffer;

use crate::polygon_contains::polygon_contains;
use super::stream::GeoStream;

use buffer::ClipBuffer;

use rejoin::rejoin;

pub trait ClipLineTrait<F> {
  fn line_start(&mut self);
  fn point(&mut self, lambda1_p : F, phi1: F);
  fn line_end(&mut self);
  fn clean(&mut self) -> Option<u8>;
}

// CompareIntersections param type!!!
struct Ci<F>
where F: Float {
  x: [F;2],
}

type InterpolateFn<F> = Box<dyn Fn(Option<F>, Option<F>, F, dyn GeoStream<F>)>;
type PointVisibleFn<F> = Box<dyn Fn(F, F) -> bool>;
type ClipLineFn<F> = Box<dyn Fn(dyn ClipLineTrait<F>)>;
type CompareIntersectionFn<F> = Box<dyn Fn(Ci<F>, Ci<F>) -> F>;

struct Clip<F>
where F: Float {
  clip_line: ClipLineFn<F>,
  interpolate: InterpolateFn<F>,

  // point: Box<dyn Fn()>,
  // point: [F;2],
  point_visible: PointVisibleFn<F>,
  polygon_started:bool,
  polygon: Option<Vec<(F,F,bool)>>,
  ring_buffer: Box<ClipBuffer<F>>,
  ring_sink: Box<dyn ClipLineTrait<F>>,
  segments: Option<Vec<(F,F,bool)>> ,
  sink: Box<dyn GeoStream<F>>,
  start: [F;2],
  ring: Vec<[F;2]>,
}

impl<F> Clip<F>
where F: Float {
  fn new(point_visible: PointVisibleFn<F>, clip_line: ClipLineFn<F>, interpolate: InterpolateFn<F>, start: [F;2], sink : Box<dyn GeoStream<F>>) -> Self {
    let ring_buffer = Box::new(ClipBuffer::<F>::new());
    return Self {
      interpolate,
      clip_line,
      // line: clipLine(sink:GeoStream),
      point_visible,
      polygon: None,
      polygon_started:false,
      ring: Vec::new(),
      ring_buffer: ring_buffer,
      ring_sink: clip_line(ring_buffer),
      segments: None,
      sink,
      start,
    };
  }

  fn point(&mut self, lambda:F, phi:F) {
    if (self.point_visible)(lambda, phi) {
      self.sink.point(lambda, phi);
    }
  }

  fn point_line(&mut self,lambda: F, phi:F) {
    (*self.clip_line.point)(lambda, phi);
  }

  fn line_start(&mut self) {
    self.clip.point = self.point_line;
    self.line.line_start();
  }

  fn line_end(&mut self) {
    self.clip.point = self.point;
    self.line.line_end();
  }



  //    fn validSegment(segment: &Vec<[f64;2]>) -> bool {
//     return segment.len() > 1;
//   }
}

impl<F> GeoStream<F> for Clip<F>
where F: Float {

     fn point(&mut self, lambda:F, phi:F) {
       self.point(lambda, phi);
     }

     fn line_start(&mut self) {
      self.line_start();
     }

    fn line_end(&mut self) {
      self.line_end();
    }

      fn polygon_start(&mut self) {
        self.point = self.pointRing;
        self.line_start = self.ringStart;
        self.line_end = self.ringEnd;

        self.segments.clear();
        self.polygon.clear();
      }

    fn polygon_end(&mut self) {
        // point = point;
        // clip.lineStart = lineStart;
        // clip.lineEnd = lineEnd;
        // segments = merge(segments);
        let  start_inside = polygon_contains(self.polygon, self.start);
        if let Some(segments) = self.segments {
          if !self.polygon_started {
            self.sink.polygon_start();
            self.polygon_started = true;
          }
          rejoin(segments, compare_intersection, start_inside, self.interpolate, self.sink);
        } else if start_inside {
          if !self.polygon_started {
            self.sink.polygon_start();
            self.polygon_started = true;
          }
          self.sink.line_start();
          (self.interpolate)(None, None, F::one(), self.sink);
          self.sink.line_end();
        }
        if self.polygon_started {
          self.sink.polygon_end();
          self.polygon_started = false;
        }
        self.segments = None;
        self.polygon = None;
      }

    fn sphere(&mut self) {
        self.sink.polygon_start();
        self.sink.line_start();
        (self.interpolate)(None, None, F::one(), self.sink);
        self.sink.line_end();
        self.sink.polygon_end();
      }

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


}


// Intersections are sorted along the clip edge. For both antimeridian cutting
// and circle clipPIng, the same comparison is used.
fn compare_intersection<F>(a: Ci<F>, b: Ci<F>) -> F
where F: Float + FloatConst {
  let a_dashed = a.x;
  let part1 = match a_dashed[0] < F::zero() {
    true => { a_dashed[1] - F::FRAC_PI_2() - F::epsilon()},
    false => { F::FRAC_PI_2() - a_dashed[1] }
  };
  let b_dashed = b.x;
  let part2 = match b_dashed[0] < F::zero() {
    true => {b_dashed[1] - F::FRAC_PI_2() - F::epsilon()},
    false => { F::FRAC_PI_2() - b_dashed[1] }
  };

  return part1 - part2;
}


// Intersections are sorted along the clip edge. For both antimeridian cutting
// and circle clipping, the same comparison is used.
// function compareIntersection(a, b) {
//   return ((a = a.x)[0] < 0 ? a[1] - halfPi - epsilon : halfPi - a[1])
//        - ((b = b.x)[0] < 0 ? b[1] - halfPi - epsilon : halfPi - b[1]);
// }
