#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod circle;
pub mod circle_generator;
pub mod circle_radius;
pub mod circle_stream;
// mod stream;

// use geo::Point;
use geo::{CoordFloat, Coordinate};

// function accepts a F value or a Function that outputs a F or maybe nothing.
pub enum FnValMaybe<T> {
    None,
    FloatValue(T),
    FloatFn(Box<dyn Fn(&CircleInArg) -> T>),
}

pub enum FnValMaybe2D<T: CoordFloat> {
    None,
    FloatValue(Coordinate<T>),
    FloatFn(Box<dyn Fn(&CircleInArg) -> Coordinate<T>>),
}

// pub struct CircleInArg
pub enum CircleInArg {
    None,
    Arg(),
}

#[derive(Clone, Debug)]
pub enum StreamType {
    Polygon,
}

pub trait CircleTrait<T: CoordFloat> {
    fn center(&mut self, center: FnValMaybe2D<T>) -> Option<Coordinate<T>>;
    fn radius(&mut self, radius: FnValMaybe<T>) -> Option<T>;
    fn precision(&mut self, precision: FnValMaybe<T>) -> Option<T>;
}
