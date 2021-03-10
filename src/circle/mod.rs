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
    // None,
    FloatValue(T),
    FloatFn(Box<dyn Fn(&CircleInArg) -> T>),
}

pub enum FnValMaybe2D<T: CoordFloat> {
    // None,
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
    fn set_center(&mut self, center: FnValMaybe2D<T>);
    fn get_center(&self, center: FnValMaybe2D<T>) -> Box<dyn Fn(&CircleInArg) -> Coordinate<T>>;
    fn set_radius(&mut self, radius: FnValMaybe<T>);
    fn get_radius(&self) -> Box<dyn Fn(&CircleInArg) -> T>;
    fn set_precision(&mut self, precision: FnValMaybe<T>);
    fn get_precision(&self) -> Box<dyn Fn(&CircleInArg) -> T>;
}
