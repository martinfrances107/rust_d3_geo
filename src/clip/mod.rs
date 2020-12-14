use std::cell::RefCell;
use std::rc::Rc;

use geo::Point;
use num_traits::{float::Float, FloatConst};

/// Public: clip generators used by projection.
pub mod antimeridian;
pub mod circle;

mod buffer;
mod rejoin;

use crate::polygon_contains::contains;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;

use buffer::ClipBuffer;
// use rejoin::rejoin;

pub trait ClipLine<'a, T> {
    fn line_start(&mut self);
    fn point(&mut self, lambda1_p: T, phi1: T, m: Option<u8>);
    fn line_end(&mut self);
    fn clean(&mut self) -> Option<u8>;
    fn stream(&mut self, stream: Box<dyn TransformStream<T>>);
}

// CompareIntersections param type
#[derive(Clone)]
pub struct Ci<T: Float> {
    x: Point<T>,
}

pub type InterpolateFn<T> =
    Box<dyn Fn(Option<Point<T>>, Option<Point<T>>, T, Rc<RefCell<Box<dyn TransformStream<T>>>>)>;
type PointsVisibleFn<T> = Box<dyn Fn(T, T, Option<T>) -> bool>;
pub type PointVisibleFnPtr<T> = Rc<PointsVisibleFn<T>>;
// type ClipLineFn<F> = dyn Fn(Box<dyn ClipLine<F>>) -> Box<dyn ClipLine<F>>;
pub type CompareIntersectionFn<T> = Box<dyn Fn(Ci<T>, Ci<T>) -> T>;

pub struct Clip<T: Float> {
    line: Rc<RefCell<Box<dyn TransformStream<T>>>>,
    interpolate: Rc<RefCell<InterpolateFn<T>>>,
    // point: Box<dyn Fn()>,
    // point: Point,
    polygon_started: bool,
    polygon: Box<Vec<Vec<Point<T>>>>,
    point_visible: PointVisibleFnPtr<T>,
    // ring_buffer: Box<dyn TransformStream>,
    ring_sink: Rc<RefCell<Box<dyn TransformStream<T>>>>,
    segments: Box<Vec<Vec<Point<T>>>>,
    start: Point<T>,
    ring: Vec<Point<T>>,
    use_ring: bool,
    sink: Rc<RefCell<Box<dyn TransformStream<T>>>>,
}

impl<'a, T: Float + FloatConst + 'static> Clip<T> {
    fn new(
        point_visible: PointVisibleFnPtr<T>,
        clip_line_fn_ptr: Rc<RefCell<StreamProcessor<T>>>,
        interpolate: Rc<RefCell<InterpolateFn<T>>>,
        start: Point<T>,
    ) -> StreamProcessor<T> {
        return Box::new(move |sink: Rc<RefCell<Box<dyn TransformStream<T>>>>| {
            let clip_line = clip_line_fn_ptr.borrow_mut();
            let line = clip_line(sink.clone());

            let ring_buffer = Rc::new(RefCell::new(ClipBuffer::new()));
            let ring_sink = clip_line(ring_buffer);

            return Rc::new(RefCell::new(Box::new(Self {
                use_ring: false,
                interpolate: interpolate.clone(),
                line,
                point_visible: point_visible.clone(),
                polygon: Box::new(Vec::new()),
                polygon_started: false,
                ring: Vec::new(),
                // ring_buffer: ring_buffer,
                ring_sink,
                segments: Box::new(Vec::new()),
                sink: sink.clone(),
                start: start.clone(),
            })));
        });
    }

    fn valid_segment(segment: &[[f64; 2]]) -> bool {
        return segment.len() > 1;
    }

    fn point_ring(&mut self, lambda: T, phi: T, _m: Option<u8>) {
        self.ring.push(Point::new(lambda, phi));
        let mut ring_sink = self.ring_sink.borrow_mut();
        ring_sink.point(lambda, phi, None);
    }

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

impl<'a, T: Float + FloatConst> TransformStream<T> for Clip<T> {
    fn point(&mut self, lambda: T, phi: T, m: Option<u8>) {
        match self.use_ring {
            true => {
                self.ring.push(Point::new(lambda, phi));
                // self.ring_sink.point(lambda, phi, None);
            }
            false => {
                if (self.point_visible)(lambda, phi, None) {
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
            // (self.interpolate)(None, None, 1f64, self.sink);
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
        // (self.interpolate)(None, None, 1f64, self.sink);
        sink.line_end();
        sink.polygon_end();
    }
}

/// Intersections are sorted along the clip edge. For both antimeridian cutting
/// and circle clipPIng, the same comparison is used.
fn compare_intersection<T: Float + FloatConst>(a: Ci<T>, b: Ci<T>) -> T {
    let a_dashed = a.x;
    let part1 = match a_dashed.x() < T::zero() {
        true => a_dashed.y() - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - a_dashed.y(),
    };
    let b_dashed = b.x;
    let part2 = match b_dashed.x() < T::zero() {
        true => b_dashed.y() - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - b_dashed.y(),
    };

    return part1 - part2;
}
