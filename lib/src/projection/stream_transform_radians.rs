use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

// A path node.
//
/// Type-Driven API, STATE prevent calls to `Self::connect()`
/// on a perviously connected object
#[derive(Clone, Debug, PartialEq)]
pub struct StreamTransformRadians<STATE, T> {
    state: STATE,
    frac_pi_180: T,
}

impl<T> Connectable for StreamTransformRadians<Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SC> = StreamTransformRadians<Connected<SC>, T>;
    #[inline]
    /// Connect this node to the next node on the path.
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC> {
        StreamTransformRadians {
            state: Connected { sink },
            frac_pi_180: self.frac_pi_180,
        }
    }
}
/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl<T> Default for StreamTransformRadians<Unconnected, T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        Self {
            state: Unconnected,
            frac_pi_180: T::PI() / T::from(180).unwrap(),
        }
    }
}

impl<EP, T, SINK> Stream for StreamTransformRadians<Connected<SINK>, T>
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
    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.state.sink.point(
            &Coord {
                x: p.x * self.frac_pi_180,
                y: p.y * self.frac_pi_180,
            },
            m,
        );
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
