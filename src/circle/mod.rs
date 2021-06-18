#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod circle;
pub mod circle_generator;
pub mod circle_radius;
pub mod circle_stream;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// function accepts a F value or a Function that outputs a F or maybe nothing.
pub enum FnValMaybe<T> {
    // None,
    FloatValue(T),
    FloatFn(Box<dyn Fn(&CircleInArg) -> T>),
}

pub enum FnValMaybe2D<T>
where
    T: CoordFloat + Default + FloatConst,
{
    // None,
    // FloatValue(Coordinate<T>),
    FloatFn(Box<dyn Fn(&CircleInArg) -> Coordinate<T>>),
}

#[derive(Debug)]
pub enum CircleInArg {
    None,
    Arg(),
}

#[derive(Clone, Debug)]
pub enum StreamType {
    Polygon,
}

pub trait CircleTrait<T: CoordFloat + Default + FloatConst> {
    fn center<'a>(self, center: Coordinate<T>) -> Self;
    fn get_center(&self) -> Coordinate<T>;
    fn radius(self, radius: T) -> Self;
    fn get_radius(&self) -> T;
    fn precision(self, precision: T) -> Self;
    fn get_precision(&self) -> T;
}
