use core::fmt::Debug;

use geo::CoordFloat;
use geo::LineString;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::rot::rotate_radians;
use crate::rot::rotate_radians::RotateRadians;
use crate::rot::rotation_identity::RotationIdentity;

use super::stream::Stream;
use super::stream_fn::stream_fn;

/// Allow for circle to be defined and then input to a stream.
#[derive(Clone, Debug)]
pub struct Generator<T>
where
    T: CoordFloat,
{
    center: Coord<T>,
    radius: T,
    precision: T,
    stream: Stream<T>,
}

impl<T> Default for Generator<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            center: Coord {
                x: T::zero(),
                y: T::zero(),
            },
            radius: T::from(90_f64).unwrap(),
            precision: T::from(2.5).unwrap(),
            stream: Stream::default(),
        }
    }
}

impl<T> Generator<T>
where
    T: CoordFloat + FloatConst,
{
    /// Injects the previously defined circle into the stream.
    pub fn circle(&mut self) -> LineString<T> {
        let c = self.center;
        let r = self.radius.to_radians();
        let p = self.precision.to_radians();

        self.stream.rotate =
            rotate_radians([-c.x.to_radians(), -c.y.to_radians(), T::zero()]);
        stream_fn(&mut self.stream, r, p, T::one(), None, None);
        let mut coordinates = vec![];
        core::mem::swap(&mut coordinates, &mut self.stream.ring);

        self.stream.rotate = RotateRadians::I(RotationIdentity::default());

        LineString(coordinates)
    }
}

impl<T> Generator<T>
where
    T: CoordFloat,
{
    /// Center is used to program the generator.
    pub fn center_set(&mut self, center: &Coord<T>) -> &mut Self {
        self.center = *center;
        self
    }

    /// Returns the currently programmed center.
    #[inline]
    pub const fn center(&self) -> Coord<T> {
        self.center
    }

    /// Sets the radius on the generator.
    pub fn radius_set(&mut self, radius: T) -> &mut Self {
        self.radius = radius;
        self
    }

    /// Returns the currently programmed radius.
    #[inline]
    pub const fn radius(&self) -> T {
        self.radius
    }

    /// Sets the precision
    ///
    /// (Number of steps in degrees )
    pub fn precision_set(&mut self, precision: T) -> &mut Self {
        self.precision = precision;
        self
    }

    /// Returns the precision
    ///
    /// (Number of steps in degrees )
    #[inline]
    pub const fn precision(&self) -> T {
        self.precision
    }
}
