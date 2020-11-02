#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod circle;
pub mod circle_radius;
pub mod circle_stream;
mod stream;

use delaunator::Point;
use std::rc::Rc;

// function accepts a F value or a Function that outputs a F or maybe nothing.
pub enum FnValMaybe {
    None,
    FloatValue(Rc<f64>),
    FloatFn(Rc<dyn Fn(CircleInArg) -> f64>),
}

pub enum FnValMaybe2D {
    None,
    FloatValue(Rc<Point>),
    FloatFn(Rc<dyn Fn(CircleInArg) -> Point>),
}

// pub struct CircleInArg
pub enum CircleInArg {
    None,
    Arg(),
}

pub trait CircleTrait {
    fn center(&mut self, center: FnValMaybe2D) -> Option<Point>;
    fn radius(&mut self, radius: FnValMaybe) -> Option<f64>;
    fn precision(&mut self, precision: FnValMaybe) -> Option<f64>;
}
