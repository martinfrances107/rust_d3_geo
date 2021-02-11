mod intersect;
mod line;

use crate::stream::Stream;
use crate::stream::StreamClipTrait;
use crate::stream::StreamInTrait;
use crate::stream::StreamSimpleNode;
use geo::{CoordFloat, Coordinate};

use crate::circle::circle_stream::circle_stream;
use crate::clip::ClipBuffer;

use super::{BufferInTrait, ClipBase};
use line::Line;
use num_traits::FloatConst;
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

        let base = ClipBase {
            line_node,
            start: Coordinate {
                x: -T::PI(),
                y: -T::FRAC_PI_2(),
            },
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
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn interpolate(
        &self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
        stream: StreamSimpleNode<T>,
    ) {
        let mut s = stream.borrow_mut();
        // circle_stream(&mut s, self.radius, self.delta, direction, from, to);
    }

    #[inline]
    fn point_visible(&self, p: Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }
}
