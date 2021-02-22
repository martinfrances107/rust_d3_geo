use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::fmt::Debug;

use crate::rotation::rotate_radians_transform::rotate_radians_transform;
use crate::stream::Stream;
use crate::stream::StreamDummy;
// use crate::stream::StreamSimpleNode;
use crate::Transform;
use crate::{cartesian::cartesian, TransformIdentity};
use crate::{cartesian::cartesian_normalize_in_place, stream::StreamIdentity};

use super::circle::CircleStream;
use super::circle_stream::circle_stream;
use super::CircleInArg;
use super::CircleTrait;
use super::FnValMaybe;
use super::FnValMaybe2D;
use super::StreamType;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CircleGenerator<T: CoordFloat> {
    center_fn: Box<dyn Fn(&CircleInArg) -> Coordinate<T>>,
    precision_fn: Box<dyn Fn(&CircleInArg) -> T>,
    radius_fn: Box<dyn Fn(&CircleInArg) -> T>,
}

impl<T: CoordFloat + FloatConst + Default + 'static> CircleGenerator<T> {
    pub fn new() -> Self {
        let center_fn = Box::new(|_in: &CircleInArg| Coordinate {
            x: T::zero(),
            y: T::zero(),
        });
        let radius_fn = Box::new(|_in: &CircleInArg| T::from(90f64).unwrap());
        let precision_fn = Box::new(|_in: &CircleInArg| T::from(6f64).unwrap());

        let c_val: Coordinate<T> = (*center_fn)(&CircleInArg::None);

        return Self {
            center_fn,
            radius_fn,
            precision_fn,
        };
    }

    pub fn circle(&self, arg: CircleInArg) -> CircleStream<T> {
        let c = (*self.center_fn)(&arg);
        let r = (*self.radius_fn)(&arg).to_radians();
        let p = (*self.precision_fn)(&arg).to_radians();

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

impl<T: CoordFloat + 'static> CircleTrait<T> for CircleGenerator<T> {
    fn center(&mut self, center: FnValMaybe2D<T>) -> Option<Coordinate<T>> {
        return match center {
            FnValMaybe2D::None => None,
            FnValMaybe2D::FloatValue(value) => {
                self.center_fn = Box::new(move |_: &CircleInArg| value);
                None
            }
            FnValMaybe2D::FloatFn(center_fn_ptr) => {
                self.center_fn = center_fn_ptr;
                None
            }
        };
    }

    fn radius(&mut self, radius: FnValMaybe<T>) -> Option<T> {
        return match radius {
            FnValMaybe::None => None,
            FnValMaybe::FloatValue(value) => {
                self.radius_fn = Box::new(move |_: &CircleInArg| value);
                None
            }
            FnValMaybe::FloatFn(radius_fn_ptr) => {
                self.radius_fn = radius_fn_ptr;
                None
            }
        };
    }

    fn precision(&mut self, precision: FnValMaybe<T>) -> Option<T> {
        match precision {
            FnValMaybe::None => None,
            FnValMaybe::FloatValue(value) => {
                self.precision_fn = Box::new(move |_: &CircleInArg| value);
                None
            }
            FnValMaybe::FloatFn(precision_fn_ptr) => {
                self.precision_fn = precision_fn_ptr;
                None
            }
        }
    }
}
