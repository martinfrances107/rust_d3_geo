#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod calc_radius;
pub mod generator;
pub mod stream;
pub mod streamFn;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// function accepts a F value or a Function that outputs a F or maybe nothing.
// pub enum FnValMaybe<T> {
//     // None,
//     FloatValue(T),
//     FloatFn(Box<dyn Fn(&InArg) -> T>),
// }

// pub enum FnValMaybe2D<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     // None,
//     // FloatValue(Coordinate<T>),
//     FloatFn(Box<dyn Fn(&InArg) -> Coordinate<T>>),
// }

// #[derive(Debug)]
// pub enum InArg {
//     None,
//     Arg(),
// }

// #[derive(Clone, Debug)]
// pub enum StreamType {
//     Polygon,
// }

// pub trait CircleTrait<T: CoordFloat + FloatConst> {
//     fn center<'a>(self, center: Coordinate<T>) -> Self;
//     fn get_center(&self) -> Coordinate<T>;
//     fn radius(self, radius: T) -> Self;
//     fn get_radius(&self) -> T;
//     fn precision(self, precision: T) -> Self;
//     fn get_precision(&self) -> T;
// }
