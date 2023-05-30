use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::rotate_radians::RotateRadians;

/// A Stream node, that applied a rotator transform.
#[derive(Debug, Clone)]
pub struct RotatorRadians<STATE, T>
where
    T: CoordFloat,
{
    state: STATE,
    rotate: RotateRadians<T>,
}

impl<T> RotatorRadians<Unconnected, T>
where
    T: CoordFloat,
{
    /// Constructor.
    #[inline]
    pub(crate) const fn new(rotate: RotateRadians<T>) -> Self {
        Self {
            state: Unconnected,
            rotate,
        }
    }
}

impl<T> Connectable for RotatorRadians<Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SINK> = RotatorRadians<Connected<SINK>, T>;
    /// Connects the next stage in the stream pipline.
    #[inline]
    fn connect<SINK>(&self, sink: SINK) -> Self::Output<SINK> {
        RotatorRadians {
            state: Connected { sink },
            rotate: self.rotate.clone(),
        }
    }
}

impl<EP, SINK, T> Stream for RotatorRadians<Connected<SINK>, T>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
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
        self.state.sink.point(&self.rotate.transform(p), m);
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
