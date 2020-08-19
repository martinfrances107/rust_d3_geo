#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod circle;
pub mod circle_radius;
pub mod circle_stream;
mod stream;

use std::rc::Rc;

// function accepts a F value or a Function that outputs a F or maybe nothing.
pub enum FnValMaybe<F> {
  None,
  FloatValue(Rc<F>),
  FloatFn(Rc<dyn Fn(CircleInArg) -> F>),
}

pub enum FnValMaybe2D<F> {
  None,
  FloatValue(Rc<[F; 2]>),
  FloatFn(Rc<dyn Fn(CircleInArg) -> [F; 2]>),
}

// pub struct CircleInArg
pub enum CircleInArg {
  None,
  Arg(),
}

pub trait CircleTrait<F> {
  fn center(&mut self, center: FnValMaybe2D<F>) -> Option<[F; 2]>;
  fn radius(&mut self, radius: FnValMaybe<F>) -> Option<F>;
  fn precision(&mut self, precision: FnValMaybe<F>) -> Option<F>;
}
