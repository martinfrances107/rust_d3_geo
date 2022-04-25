use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::line_string;
use geo::LineString;
use geo::Polygon;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
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
    center: Coordinate<T>,
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
            center: Coordinate {
                x: T::zero(),
                y: T::zero(),
            },
            radius: T::from(90_f64).unwrap(),
            precision: T::from(6).unwrap(),
            stream: Stream::default(),
        }
    }
}

impl<T> Generator<T>
where
    T: CoordFloat + FloatConst,
{
    /// Injects the previously defined circle into the stream.
    pub fn circle(&mut self) -> Polygon<T> {
        let c = self.center;
        let r = self.radius.to_radians();
        let p = self.precision.to_radians();

        self.stream.rotate = rotate_radians([-c.x.to_radians(), -c.y.to_radians(), T::zero()]);
        stream_fn(&mut self.stream, r, p, T::one(), None, None);
        let coordinates = self.stream.ring.clone();

        let polygon = Polygon::new(LineString(coordinates), vec![]);

        self.stream.ring.clear();
        self.stream.rotate = RotateRadians::I(RotationIdentity::default());
        polygon
    }
}

impl<T> Generator<T>
where
    T: CoordFloat,
{
    /// Center is used to programe the generator.
    pub fn center(mut self, center: &Coordinate<T>) -> Generator<T> {
        self.center = *center;
        self
    }

    /// Returns the currently programmed center.
    #[inline]
    pub fn get_center(&self) -> Coordinate<T> {
        self.center
    }

    /// Sets the radius on the generator.
    pub fn radius(mut self, radius: T) -> Self {
        self.radius = radius;
        self
    }

    /// Returns the currently programmed radius.
    #[inline]
    pub fn get_radius(&self) -> T {
        self.radius
    }

    /// Sets the precision.
    pub fn precision(mut self, precision: T) -> Self {
        self.precision = precision;
        self
    }

    /// Returns the precision.
    #[inline]
    pub fn get_precision(&self) -> T {
        self.precision
    }
}
