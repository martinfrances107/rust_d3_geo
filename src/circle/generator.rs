use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::rotation::rotate_radians;
use crate::rotation::rotate_radians::RotateRadians;
use crate::rotation::rotation_identity::RotationIdentity;

use super::stream::Stream;
use super::stream_fn::stream_fn;

/// Allow for circle to be defined and then input to a stream.
#[derive(Debug)]
pub struct Generator<T>
where
    T: CoordFloat + FloatConst,
{
    center: Coordinate<T>,
    radius: T,
    precision: T,
    stream: Rc<RefCell<Stream<T>>>,
}

impl<T> Default for Generator<T>
where
    T: CoordFloat + FloatConst,
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
            stream: Rc::new(RefCell::new(Stream::default())),
        }
    }
}

impl<T> Generator<T>
where
    T: CoordFloat + FloatConst,
{
    /// Injects the previously defined circle into the stream.
    pub fn circle(&self) -> Vec<Vec<Coordinate<T>>> {
        let c = self.center;
        let r = self.radius.to_radians();
        let p = self.precision.to_radians();

        self.stream.borrow_mut().rotate =
            rotate_radians([-c.x.to_radians(), -c.y.to_radians(), T::zero()]);

        // let mut cs = Rc::new(RefCell::new(CircleStream {
        //     ring: Vec::new(),
        //     rotate,
        //     stream_type: StreamType::Polygon,
        //     coordinates: vec![vec![]],
        // }));

        stream_fn(self.stream.clone(), r, p, T::one(), None, None);

        let coordinates = vec![self.stream.borrow().ring.clone()];

        let mut stream_b = self.stream.borrow_mut();
        stream_b.ring.clear();
        stream_b.rotate = RotateRadians::I(RotationIdentity::default());

        coordinates
    }
}

impl<T> Generator<T>
where
    T: CoordFloat + FloatConst,
{
    /// center is use to programe the generator.
    pub fn center(mut self, center: &Coordinate<T>) -> Generator<T> {
        self.center = *center;
        self
    }

    /// Returns the currently programmed center.
    #[inline]
    pub fn get_center(&self) -> Coordinate<T> {
        self.center
    }

    /// radius used to programe the generator.
    pub fn radius(mut self, radius: T) -> Self {
        self.radius = radius;
        self
    }

    /// Returns the currently programmed radius.
    #[inline]
    pub fn get_radius(&self) -> T {
        self.radius
    }

    fn precision(mut self, precision: T) -> Self {
        self.precision = precision;
        self
    }

    #[inline]
    fn get_precision(&self) -> T {
        self.precision
    }
}
