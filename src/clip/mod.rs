use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

pub mod antimeridian;
mod buffer;
mod rejoin;

use super::stream::Stream;
use crate::polygon_contains::contains;
use crate::transform_stream::TransformStream;

use buffer::ClipBuffer;
// use rejoin::rejoin;

pub trait ClipLine<'a, F> {
  fn line_start(&mut self);
  fn point(&mut self, lambda1_p: F, phi1: F, m: Option<F>);
  fn line_end(&mut self);
  fn clean(&mut self) -> Option<u8>;
  fn stream(&mut self, stream: Box<dyn TransformStream<F>>);
}

// CompareIntersections param type
#[derive(Clone, Copy, Debug)]
pub struct Ci<F>
where
  F: Float,
{
  x: [F; 2],
}

pub type InterpolateFn<F> = Box<dyn Fn(Option<F>, Option<F>, F, dyn Stream<F>)>;
type ClipLineFn<F> = Box<dyn Fn(F, F, Option<F>) -> bool>;
type PointVisibleFnPtr<F> = Rc<ClipLineFn<F>>;
// type ClipLineFn<F> = dyn Fn(Box<dyn ClipLine<F>>) -> Box<dyn ClipLine<F>>;
pub type CompareIntersectionFn<F> = Box<dyn Fn(Ci<F>, Ci<F>) -> F>;

pub struct Clip<F>
where
  F: Float,
{
  line: Box<dyn TransformStream<F>>,
  interpolate: Rc<RefCell<Box<dyn TransformStream<F>>>>,
  // point: Box<dyn Fn()>,
  // point: [F;2],
  polygon_started: bool,
  polygon: Box<Vec<Vec<[F; 2]>>>,
  point_visible: PointVisibleFnPtr<F>,
  // ring_buffer: Box<dyn TransformStream<F>>,
  // ring_sink: Box<dyn TransformStream<F>>,
  segments: Box<Vec<Vec<[F; 2]>>>,
  start: [F; 2],
  ring: Vec<[F; 2]>,
  use_ring: bool,
  sink: Rc<RefCell<Box<dyn TransformStream<F>>>>,
}

impl<'a, F> Clip<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  fn new(
    point_visible: PointVisibleFnPtr<F>,
    clip_line_fn_ptr: Rc<RefCell<Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Box<dyn TransformStream<F>>>>>,
    interpolate: Rc<RefCell<Box<dyn TransformStream<F>>>>,
    start: [F; 2],
  ) -> Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Box<dyn TransformStream<F>>> {

    return Box::new(move |sink_ptr: Rc<RefCell<Box<dyn TransformStream<F>>>>| {
      // let interpolate = interpolate.clone();
      let sink1 = sink_ptr.clone();
      // let mut clip_line_fn: Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Box<dyn TransformStream<F>>>;
      let clip_line = clip_line_fn_ptr.borrow_mut();
      let line = clip_line(sink1);



      let sink2 = sink_ptr.clone();

      // let point_visible: Rc<PointVisibleFn<F>> = point_visible_ptr;
      // let ring_buffer = Box::new(ClipBuffer::<F>::new());
      // let ring_sink = clip_line;
      // // ring_sink.stream(&ring_buffer);
      // // clip_line.stream(ring_buffer);
      return Box::new(Self {
        use_ring: false,
        interpolate: interpolate.clone(),
        // ring_sink: ring_sink,
        line,
        point_visible: point_visible.clone(),
        polygon: Box::new(Vec::new()),
        polygon_started: false,
        ring: Vec::new(),
        // ring_buffer: ring_buffer,
        segments: Box::new(Vec::new()),
        sink:sink2,
        start,
      });
    });
  }

  fn valid_segment(segment: &Vec<[f64; 2]>) -> bool {
    return segment.len() > 1;
  }

  fn point_ring(&self, _lambda: F, _phi: F, _m: Option<F>) {}

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

impl<'a, F> TransformStream<F> for Clip<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn point(&mut self, lambda: F, phi: F, m: Option<F>) {
    match self.use_ring {
      true => {
        self.ring.push([lambda, phi]);
        // self.ring_sink.point(lambda, phi, None);
      }
      false => {
        if (self.point_visible)(lambda, phi, m) {
          let mut sink = self.sink.borrow_mut();
          sink.point(lambda, phi, m);
        }
      }
    }
  }

  fn line_start(&mut self) {
    // self.clip.point = self.point_line;
    // self.line.line_start();
  }

  fn line_end(&mut self) {
    // self.line_end();
  }

  fn polygon_start(&mut self) {
    // self.point = self.pointRing;
    // self.line_start = self.ringStart;
    // self.line_end = self.ringEnd;
    self.use_ring = true;
    self.segments.clear();
    self.polygon.clear();
  }

  fn polygon_end(&mut self) {
    self.use_ring = false;
    // point = point;
    // clip.lineStart = lineStart;
    // clip.lineEnd = lineEnd;
    // segments = merge(segments);
    let start_inside = contains(self.polygon.to_vec(), &self.start);
    let mut sink = self.sink.borrow_mut();
    if !self.polygon_started {
      let mut sink = self.sink.borrow_mut();
      sink.polygon_start();
      self.polygon_started = true;

    // rejoin(self.segments.to_vec(), Box::new(compare_intersection), start_inside, self.interpolate, self.sink);
    } else if start_inside {
      if !self.polygon_started {
        sink.polygon_start();
        self.polygon_started = true;
      }
      sink.line_start();
      // (self.interpolate)(None, None, F::one(), self.sink);
      sink.line_end();
    }
    if self.polygon_started {
      sink.polygon_end();
      self.polygon_started = false;
    }
    self.segments.clear();
    self.polygon.clear();
  }

  fn sphere(&mut self) {
    let mut sink = self.sink.borrow_mut();
    sink.polygon_start();
    sink.line_start();
    // (self.interpolate)(None, None, F::one(), self.sink);
    sink.line_end();
    sink.polygon_end();
  }
}

/// Intersections are sorted along the clip edge. For both antimeridian cutting
/// and circle clipPIng, the same comparison is used.
fn compare_intersection<F>(a: Ci<F>, b: Ci<F>) -> F
where
  F: Float + FloatConst,
{
  let a_dashed = a.x;
  let part1 = match a_dashed[0] < F::zero() {
    true => a_dashed[1] - F::FRAC_PI_2() - F::epsilon(),
    false => F::FRAC_PI_2() - a_dashed[1],
  };
  let b_dashed = b.x;
  let part2 = match b_dashed[0] < F::zero() {
    true => b_dashed[1] - F::FRAC_PI_2() - F::epsilon(),
    false => F::FRAC_PI_2() - b_dashed[1],
  };

  return part1 - part2;
}
// Intersections are sorted along the clip edge. For both antimeridian cutting
// and circle clipping, the same comparison is used.
// function compareIntersection(a, b) {
//   return ((a = a.x)[0] < 0 ? a[1] - halfPi - epsilon : halfPi - a[1])
//        - ((b = b.x)[0] < 0 ? b[1] - halfPi - epsilon : halfPi - b[1]);
// }
