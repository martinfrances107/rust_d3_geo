use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

/// Identity is a stream pipe line stage.
/// that acts as a pass through node.
#[derive(Clone, Debug)]
pub struct Identity<STATE> {
    state: STATE,
}

/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl Default for Identity<Unconnected> {
    #[inline]
    fn default() -> Self {
        Self { state: Unconnected }
    }
}

impl Connectable for Identity<Unconnected> {
    /// The resultant builder type.
    type Output<SC> = Identity<Connected<SC>>;

    #[inline]
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC> {
        Identity {
            state: Connected { sink },
        }
    }
}

impl<EP, SINK, T> Stream for Identity<Connected<SINK>>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    #[inline]
    fn line_end(&mut self) {
        self.state.sink.line_end();
    }

    #[inline]
    fn line_start(&mut self) {
        self.state.sink.line_start();
    }

    #[inline]
    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        self.state.sink.point(p, m);
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.state.sink.polygon_start();
    }

    #[inline]
    fn sphere(&mut self) {
        self.state.sink.sphere();
    }
}
