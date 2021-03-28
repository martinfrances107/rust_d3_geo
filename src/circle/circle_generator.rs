use std::fmt::Debug;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::rotation::rotate_radians_transform::rotate_radians_transform;
use crate::stream::Stream;
// use crate::stream::StreamSimpleNode;
use super::circle::CircleStream;
use super::circle_stream::circle_stream;
use super::CircleInArg;
use super::CircleTrait;
use super::FnValMaybe;
use super::FnValMaybe2D;
use super::StreamType;

use crate::cartesian::cartesian_normalize_in_place;
// use crate::stream::stream_identity::StreamIdentity;
use crate::Transform;
use crate::{cartesian::cartesian, TransformIdentity};
use std::cell::RefCell;
use std::rc::Rc;

// pub fn constant<T: 'static + Copy>(x: T) -> Box<dyn Fn(&CircleInArg) -> T> {
//     Box::new(move |_| x)
// }

pub struct CircleGenerator<T>
where
    T: CoordFloat + FloatConst + Default,
{
    // pub center: Box<dyn Fn(&CircleInArg) -> Coordinate<T> + 'a>,
    // pub precision: Box<dyn Fn(&CircleInArg) -> T + 'a>,
    // pub radius: Box<dyn Fn(&CircleInArg) -> T + 'a>,
    center: Coordinate<T>,
    radius: T,
    precision: T,
}

impl<T> Default for CircleGenerator<T>
where
    T: CoordFloat + FloatConst + Default,
{
    #[inline]
    fn default() -> Self {
        return Self {
            center: Coordinate {
                x: T::zero(),
                y: T::zero(),
            },
            radius: T::from(90f64).unwrap(),
            precision: T::from(6).unwrap(),
        };
    }
}

impl<T> CircleGenerator<T>
where
    T: AddAssign + CoordFloat + FloatConst + Default,
{
    pub fn circle(&self) -> CircleStream<T> {
        let c = self.center;
        let r = self.radius.to_radians();
        let p = self.precision.to_radians();
        let rotate = rotate_radians_transform(-c.x.to_radians(), -c.y.to_radians(), T::zero());

        let mut cs = CircleStream {
            ring: Vec::new(),
            rotate,
            stream_type: StreamType::Polygon,
            coordinates: vec![vec![]],
        };

        circle_stream(&mut cs, r, p, T::one(), None, None);

        // Finialise.
        // - TODO can I remove this clone.
        cs.coordinates = vec![cs.ring.clone()];

        cs
    }
}

impl<T> CircleTrait<T> for CircleGenerator<T>
where
    T: CoordFloat + FloatConst + Default,
{
    fn center(mut self, center: Coordinate<T>) -> CircleGenerator<T> {
        self.center = center;
        self
    }

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        self.center
    }

    fn radius(mut self, radius: T) -> Self {
        self.radius = radius;
        self
    }

    #[inline]
    fn get_radius(&self) -> T {
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
