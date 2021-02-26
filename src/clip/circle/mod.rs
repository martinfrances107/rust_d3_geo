mod intersect;
mod line;
use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::circle::circle_stream::circle_stream;
use crate::clip::ClipBuffer;
use crate::stream::CompareIntersection;
use crate::stream::Stream;
use crate::stream::StreamClipTrait;
use crate::stream::StreamClone;
use crate::stream::StreamInTrait;
use crate::stream::StreamSimpleNode;

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
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn new(radius: T) -> Self {
        let cr = radius.cos();
        let delta = T::from(6u8).unwrap().to_radians();

        let line_node = Line::new(radius);
        let ring_buffer_node = ClipBuffer::default();
        let ring_sink_node = Line::new(radius);
        // let mut rc = ring_sink_node.borrow_mut();
        ring_sink_node.buffer_in(ring_buffer_node.clone());

        let interpolate = Box::new(
            move |from: Option<Coordinate<T>>,
                  to: Option<Coordinate<T>>,
                  direction: T,
                  stream: &mut dyn Stream<ScC = Coordinate<T>>| {
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
impl<T> StreamClone for ClipCircle<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(*self.clone())
    }
}
impl<T> Stream for ClipCircle<T> where T: CoordFloat + FloatConst + 'static {}

impl<T> StreamClipTrait for ClipCircle<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type SctOC = Option<Coordinate<T>>;
    type SctStream = StreamSimpleNode<T>;
    type SctT = T;
    type SctCi = CompareIntersection<T>;

    #[inline]
    fn point_visible(&self, p: Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }

    fn interpolate(
        &self,
        _from: Self::SctOC,
        _to: Self::SctOC,
        _direction: Self::SctT,
        _stream: Self::SctStream,
    ) {
        panic!("dummmy function");
    }
}
