use std::fmt::Debug;

use geo::{CoordFloat, Coordinate};

use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

/// TODO: Can this be optimised away?
#[derive(Clone, Debug)]
pub struct StreamTransformRadians<STATE>(pub STATE);

impl StreamTransformRadians<Unconnected> {
    pub fn connect<EP, SINK, T>(self, sink: SINK) -> StreamTransformRadians<Connected<SINK>> {
        StreamTransformRadians(Connected { sink })
    }
}
/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl Default for StreamTransformRadians<Unconnected> {
    fn default() -> Self {
        Self(Unconnected)
    }
}

impl<EP, T, SINK> Stream for StreamTransformRadians<Connected<SINK>>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.0.sink.endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.0.sink.point(
            &Coordinate {
                x: p.x.to_radians(),
                y: p.y.to_radians(),
            },
            m,
        );
    }

    #[inline]
    fn sphere(&mut self) {
        self.0.sink.sphere();
    }

    #[inline]
    fn line_start(&mut self) {
        self.0.sink.line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.0.sink.line_end();
    }
    #[inline]
    fn polygon_start(&mut self) {
        self.0.sink.polygon_start()
    }
    #[inline]
    fn polygon_end(&mut self) {
        self.0.sink.polygon_end();
    }
}
