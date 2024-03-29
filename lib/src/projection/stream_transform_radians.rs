use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

// A path node.
//
/// Type-Driven API, STATE prevent calls to `Self::connect()`
/// on a perviously connected object
#[derive(Clone, Debug)]
pub struct StreamTransformRadians<STATE>(pub STATE);

impl Connectable for StreamTransformRadians<Unconnected> {
    type Output<SC> = StreamTransformRadians<Connected<SC>>;
    #[inline]
    /// Connect this node to the next node on the path.
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC> {
        StreamTransformRadians(Connected { sink })
    }
}
/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl Default for StreamTransformRadians<Unconnected> {
    #[inline]
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
    fn line_end(&mut self) {
        self.0.sink.line_end();
    }

    #[inline]
    fn line_start(&mut self) {
        self.0.sink.line_start();
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.0.sink.point(
            &Coord {
                x: p.x.to_radians(),
                y: p.y.to_radians(),
            },
            m,
        );
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.0.sink.polygon_end();
    }
    #[inline]
    fn polygon_start(&mut self) {
        self.0.sink.polygon_start();
    }
    #[inline]
    fn sphere(&mut self) {
        self.0.sink.sphere();
    }
}
