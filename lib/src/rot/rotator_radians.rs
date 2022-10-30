use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

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
    pub fn new(rotate: RotateRadians<T>) -> Self {
        Self {
            state: Unconnected,
            rotate,
        }
    }
}

impl<T> RotatorRadians<Unconnected, T>
where
    T: CoordFloat,
{
    /// Connects the next stage in the stream pipline.
    #[inline]
    pub fn connect<SINK>(self, sink: SINK) -> RotatorRadians<Connected<SINK>, T>
    where
        SINK: Clone,
    {
        RotatorRadians {
            state: Connected { sink },
            rotate: self.rotate,
        }
    }
}

impl<EP, SINK, T> Stream for RotatorRadians<Connected<SINK>, T>
where
    SINK: Clone + Stream<EP = EP, T = T>,
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
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        self.state.sink.point(&self.rotate.transform(p), m);
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.state.sink.polygon_start()
    }

    #[inline]
    fn sphere(&mut self) {
        self.state.sink.sphere();
    }
}
