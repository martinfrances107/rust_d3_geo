mod intersect;
mod line;
use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::circle::circle_stream::circle_stream;
use crate::clip::ClipBuffer;
use crate::stream::Stream;
use crate::stream::StreamClipTrait;
use crate::stream::StreamInTrait;

use super::clip_base::ClipBase;
use super::BufferInTrait;

use line::Line;
pub struct ClipCircle<T: CoordFloat + FloatConst> {
    radius: T,
    delta: T,
    cr: T,
    base: ClipBase<T>,
}

impl<T> StreamInTrait<T> for ClipCircle<T> where T: CoordFloat + FloatConst {}

/// Returns a clip object
impl<T> ClipCircle<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    fn new(radius: T) -> Self {
        let cr = radius.cos();
        let delta = T::from(6u8).unwrap().to_radians();

        let line_node = Line::gen_node(radius);
        let ring_buffer_node = ClipBuffer::gen_node();
        let ring_sink_node = Line::gen_node(radius);
        let mut rc = ring_sink_node.borrow_mut();
        rc.buffer_in(ring_buffer_node.clone());

        let interpolate = Box::new(
            move |from: Option<Coordinate<T>>,
                  to: Option<Coordinate<T>>,
                  direction: T,
                  stream: &mut dyn Stream<T>| {
                circle_stream(stream, radius, delta, direction, from, to);
            },
        );

        let base = ClipBase {
            line_node,
            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
            interpolate,
            ring_buffer_node,
            // ring_sink_node,
            ..ClipBase::default()
        };

        Self {
            radius,
            delta,
            cr,
            base,
        }
    }
}
impl<T> Stream<T> for ClipCircle<T> where T: CoordFloat + FloatConst {}

impl<T> StreamClipTrait<T> for ClipCircle<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    #[inline]
    fn point_visible(&self, p: Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }
}
