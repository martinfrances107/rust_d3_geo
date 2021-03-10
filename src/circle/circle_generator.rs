use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::fmt::Debug;

use crate::rotation::rotate_radians_transform::rotate_radians_transform;
use crate::stream::stream_dummy::StreamDummy;
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
use crate::stream::stream_identity::StreamIdentity;
use crate::Transform;
use crate::{cartesian::cartesian, TransformIdentity};
use std::cell::RefCell;
use std::rc::Rc;

pub fn constant<T: Copy + 'static>(x: T) -> Box<dyn Fn(&CircleInArg) -> T> {
    Box::new(move |_| x)
}

pub struct CircleGenerator<T: CoordFloat> {
    pub center: Box<dyn Fn(&CircleInArg) -> Coordinate<T>>,
    pub precision: Box<dyn Fn(&CircleInArg) -> T>,
    pub radius: Box<dyn Fn(&CircleInArg) -> T>,
}

impl<T: CoordFloat + FloatConst + Default + 'static> Default for CircleGenerator<T> {
    #[inline]
    fn default() -> Self {
        return Self {
            center: constant(Coordinate {
                x: T::zero(),
                y: T::zero(),
            }),
            radius: constant(T::from(90f64).unwrap()),
            precision: constant(T::from(6f64).unwrap()),
        };
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> CircleGenerator<T> {
    pub fn circle(&self, arg: &CircleInArg) -> Box<CircleStream<T>> {
        let c = (self.center)(arg);
        let r = (self.radius)(arg).to_radians();
        let p = (self.precision)(arg).to_radians();

        let rotate = rotate_radians_transform(-c.x.to_radians(), -c.y.to_radians(), T::zero());

        let cs = Box::new(CircleStream {
            ring: Vec::new(),
            rotate,
            stream_type: StreamType::Polygon,
            coordinates: vec![vec![]],
        });

        // {
        //     let mut ot: Box<dyn Stream<C = Coordinate<T>>> = cs;
        //     circle_stream(&mut ot, r, p, T::one(), None, None);
        // }

        // Finialise.
        // - TODO can I remove this clone.
        // cs.coordinates = vec![cs.ring.clone()];

        cs
    }
}

impl<T: CoordFloat + 'static> CircleTrait<T> for CircleGenerator<T> {
    fn set_center(&mut self, center: FnValMaybe2D<T>) {
        return match center {
            FnValMaybe2D::FloatValue(value) => {
                self.center = constant(value);
            }
            FnValMaybe2D::FloatFn(center) => {
                self.center = center;
            }
        };
    }

    #[inline]
    fn get_center(&self, center: FnValMaybe2D<T>) -> Box<dyn Fn(&CircleInArg) -> Coordinate<T>> {
        // self.center
        panic!("must clone")
    }

    fn set_radius(&mut self, radius: FnValMaybe<T>) {
        match radius {
            FnValMaybe::FloatValue(value) => {
                self.radius = constant(value);
            }
            FnValMaybe::FloatFn(radius) => {
                self.radius = radius;
            }
        };
    }

    #[inline]
    fn get_radius(&self) -> Box<dyn Fn(&CircleInArg) -> T> {
        panic!("must clone")
        // self.radius
    }

    fn set_precision(&mut self, precision: FnValMaybe<T>) {
        match precision {
            FnValMaybe::FloatValue(value) => {
                self.precision = constant(value);
            }
            FnValMaybe::FloatFn(precision) => {
                self.precision = precision;
            }
        }
    }

    #[inline]
    fn get_precision(&self) -> Box<dyn Fn(&CircleInArg) -> T> {
        panic!("must clone")
        // self.precision
    }
}
