pub mod antimeridian;
mod buffer;
pub mod circle;
// use std::fmt::Debug;
use crate::stream::StreamNode;
// use crate::Transform;
use crate::{stream::Stream, transform_stream::StreamProcessor};
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::{cell::RefCell, rc::Rc};

use crate::polygon_contains::contains;

use buffer::ClipBuffer;

// // CompareIntersections param type
#[derive(Clone, Debug)]
pub struct Ci<T: CoordFloat> {
    x: Coordinate<T>,
}

pub type InterpolateFn<T> =
    Rc<Box<dyn Fn(Option<Coordinate<T>>, Option<Coordinate<T>>, T, StreamNode<T>)>>;
type PointVisibleFn<T> = Rc<Box<dyn Fn(T, T, Option<u8>) -> bool>>;

pub struct Clip<T: CoordFloat + 'static> {
    line: StreamNode<T>,
    interpolate: InterpolateFn<T>,
    polygon_started: bool,
    polygon: Vec<Vec<Coordinate<T>>>,
    point_visible: PointVisibleFn<T>,
    ring_buffer: StreamNode<T>,
    ring_sink: StreamNode<T>,
    segments: Vec<Vec<Coordinate<T>>>,
    start: Coordinate<T>,
    ring: Vec<Coordinate<T>>,
    use_ring: bool,
    sink: StreamNode<T>,
}

impl<T: CoordFloat + FloatConst + 'static> Clip<T> {
    fn gen_stream_processor(
        point_visible: PointVisibleFn<T>,
        clip_line: StreamProcessor<T>,
        interpolate: InterpolateFn<T>,
        start: Coordinate<T>,
    ) -> StreamProcessor<T> {
        Box::new(move |sink: StreamNode<T>| {
            let line = clip_line(sink.clone());

            let ring_buffer = ClipBuffer::<T>::new();
            let ring_sink = clip_line(ring_buffer.clone());

            Rc::new(RefCell::new(Box::new(Self {
                use_ring: false,
                interpolate: interpolate.clone(),
                line,
                point_visible: point_visible.clone(),
                polygon: Vec::new(),
                polygon_started: false,
                ring: Vec::new(),
                ring_buffer: ring_buffer.clone(),
                ring_sink,
                segments: Vec::new(),
                sink: sink.clone(),
                start: start.clone(),
            })))
        })
    }

    #[inline]
    fn valid_segment(segment: &[[f64; 2]]) -> bool {
        segment.len() > 1
    }

    fn point_ring(&mut self, lambda: T, phi: T, _m: Option<u8>) {
        self.ring.push(Coordinate { x: lambda, y: phi });
        let mut rs = self.ring_sink.borrow_mut();
        rs.point(lambda, phi, None);
    }

    fn ring_start(&mut self) {
        let mut sink = self.ring_sink.borrow_mut();
        sink.line_start();
        self.ring = Vec::new();
    }

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

impl<T: CoordFloat + FloatConst> Stream<T> for Clip<T> {
    fn point(&mut self, lambda: T, phi: T, m: Option<u8>) {
        match self.use_ring {
            true => {
                self.ring.push(Coordinate { x: lambda, y: phi });
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
        let start_inside = contains(&self.polygon, &self.start);
        let mut sink = self.sink.borrow_mut();
        if !self.polygon_started {
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
fn compare_intersection<T: CoordFloat + FloatConst>(a: Ci<T>, b: Ci<T>) -> T {
    let a_dashed = a.x;
    let part1 = match a_dashed.x < T::zero() {
        true => a_dashed.y - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - a_dashed.y,
    };
    let b_dashed = b.x;
    let part2 = match b_dashed.x < T::zero() {
        true => b_dashed.y - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - b_dashed.y,
    };

    return part1 - part2;
}
