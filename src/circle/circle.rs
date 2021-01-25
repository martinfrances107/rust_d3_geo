use std::fmt::Debug;
// use geo::Point;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::cartesian::cartesian_normalize_in_place;
use crate::rotation::rotate_radians::RotateRadians;
use crate::stream::Stream;
use crate::Transform;
use crate::{cartesian::cartesian, TransformIdentity};

use super::circle_stream::circle_stream;
use super::CircleInArg;
use super::CircleTrait;
use super::FnValMaybe;
use super::FnValMaybe2D;

#[derive(Debug)]
enum StreamType {
    Polygon,
}

/// Output of Circle::circle()
#[derive(Debug)]
pub struct CircleStream<T: CoordFloat> {
    stream_type: StreamType,
    pub coordinates: Vec<Vec<Coordinate<T>>>,
}

// #[derive(Clone)]
pub struct Circle<T: CoordFloat> {
    center_fn: Box<dyn Fn(&CircleInArg) -> Coordinate<T>>,
    precision_fn: Box<dyn Fn(&CircleInArg) -> T>,
    radius_fn: Box<dyn Fn(&CircleInArg) -> T>,
    rotate: Box<dyn Transform<T>>,
    ring: Vec<Coordinate<T>>,
}

impl<T: CoordFloat + FloatConst + 'static> Circle<T> {
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
            rotate: Box::new(TransformIdentity {}),
            ring: Vec::new(),
        };
    }

    pub fn circle(&mut self, arg: CircleInArg) -> CircleStream<T> {
        let c = (*self.center_fn)(&arg);
        let r = (*self.radius_fn)(&arg).to_radians();
        let p = (*self.precision_fn)(&arg).to_radians();
        println!("c {:?} {:?}", c.x, c.y);
        println!("r {:?}", r);
        println!("p {:?}", p);
        self.rotate = RotateRadians::new(-c.x.to_radians(), -c.y.to_radians(), T::zero());

        circle_stream(self, r, p, T::one(), None, None);

        let c;
        {
            let mut coordinates = Vec::new();
            coordinates.push(self.ring.to_vec());

            c = CircleStream {
                stream_type: StreamType::Polygon,
                coordinates,
            };
        }

        return c;
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for Circle<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let x_rotated = self.rotate.invert(&Coordinate { x, y });
        let x_rotated_deg = Coordinate {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        };
        self.ring.push(x_rotated_deg);
    }
}

impl<T: CoordFloat + 'static> CircleTrait<T> for Circle<T> {
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
