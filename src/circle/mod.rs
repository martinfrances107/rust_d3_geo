#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod circle;
pub mod circle_radius;
pub mod circle_stream;
mod stream;

use geo::Point;
use num_traits::Float;
use std::rc::Rc;

// function accepts a F value or a Function that outputs a F or maybe nothing.
pub enum FnValMaybe<T> {
    None,
    FloatValue(Rc<T>),
    FloatFn(Rc<dyn Fn(CircleInArg) -> T>),
}

pub enum FnValMaybe2D<T: Float> {
    None,
    FloatValue(Rc<Point<T>>),
    FloatFn(Rc<dyn Fn(CircleInArg) -> Point<T>>),
}

// pub struct CircleInArg
pub enum CircleInArg {
    None,
    Arg(),
}

pub trait CircleTrait<T: Float> {
    fn center(&mut self, center: FnValMaybe2D<T>) -> Option<Point<T>>;
    fn radius(&mut self, radius: FnValMaybe<T>) -> Option<T>;
    fn precision(&mut self, precision: FnValMaybe<T>) -> Option<T>;
}
