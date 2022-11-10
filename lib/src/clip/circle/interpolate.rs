use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::circle::stream_fn::stream_fn;
use crate::clip::Interpolator;
use crate::stream::Stream;

/// Interpolate Circle.
#[derive(Clone, Debug)]
pub struct Interpolate<T> {
    pub(crate) radius: T,
    delta: T,
}

impl<T> Interpolate<T>
where
    T: CoordFloat + FloatConst,
{
    /// Constructs a Interpolate State based on the specified radius.
    ///
    /// # Panics
    ///  Will never happen as 6 will always be converted into T.
    pub fn new(radius: T) -> Self {
        Self {
            radius,
            delta: T::from(6_f64).unwrap().to_radians(),
        }
    }
}

impl<T> Interpolator for Interpolate<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn interpolate<EP, STREAM>(
        &self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
        stream: &mut STREAM,
    ) where
        STREAM: Stream<EP = EP, T = T>,
    {
        stream_fn(stream, self.radius, self.delta, direction, from, to);
    }
}
