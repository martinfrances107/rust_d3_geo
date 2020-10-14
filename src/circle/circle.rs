use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use delaunator::Point;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::rotation::rotate_radians::RotateRadians;
use crate::transform_stream::TransformStream;
use crate::Transform;

use super::circle_stream::circle_stream;
use super::stream::Stream;
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
pub struct CircleStream {
  stream_type: StreamType,
  pub coordinates: Vec<Vec<Point>>,
}

#[derive(Clone)]
pub struct Circle
{
  center_fn_ptr: Rc<dyn Fn(CircleInArg) -> Point>,
  radius_fn_ptr: Rc<dyn Fn(CircleInArg) -> f64>,
  precision_fn_ptr: Rc<dyn Fn(CircleInArg) -> f64>,
}

fn center(_in: CircleInArg) -> Point
{
  return Point{x:0f64, y:0f64};
}

fn radius(_in: CircleInArg) -> f64
{
  return 90f64;
}

fn precision(_in: CircleInArg) -> f64
{
  return 6f64
}

impl Circle
{
  pub fn new() -> Self {
    let center_fn_ptr = Rc::new(center);
    let radius_fn_ptr = Rc::new(radius);
    let precision_fn_ptr = Rc::new(precision);

    let c_val: Point = (*center_fn_ptr)(CircleInArg::None);

    return Self {
      center_fn_ptr,
      radius_fn_ptr,
      precision_fn_ptr,
    };
  }

  pub fn circle(&mut self, _arg: CircleInArg) -> CircleStream {
    // TODO must come back and copy the arg so in can be passes into each fn c, r and p.
    let c = (*self.center_fn_ptr)(CircleInArg::None);
    let r = (*self.radius_fn_ptr)(CircleInArg::None).to_radians();
    let p = (*self.precision_fn_ptr)(CircleInArg::None).to_radians();

    let ring = Rc::new(RefCell::new(Vec::new()));

    let rotate = Rc::new(RotateRadians::new(
      -c.x.to_radians(),
      -c.y.to_radians(),
      0f64,
    ));

    let stream = Rc::new(RefCell::new(Stream::new(rotate.clone(), ring.clone())));

    circle_stream(stream, r, p, 1f64, None, None);

    let c;
    {
      let ring = ring.borrow_mut();
      let mut coordinates = Vec::new();
      coordinates.push(ring.to_vec());

      c = CircleStream {
        stream_type: StreamType::Polygon,
        coordinates,
      };
    }

    return c;
  }
}

impl CircleTrait for Circle
{
  fn center(&mut self, center: FnValMaybe2D) -> Option<Point> {
    return match center {
      FnValMaybe2D::None => None,
      FnValMaybe2D::FloatValue(value) => {
        self.center_fn_ptr = Rc::new(move |_: CircleInArg| (*value).clone());
        None
      }
      FnValMaybe2D::FloatFn(center_fn_ptr) => {
        self.center_fn_ptr = center_fn_ptr;
        None
      }
    };
  }

  fn radius(&mut self, radius: FnValMaybe) -> Option<f64> {
    return match radius {
      FnValMaybe::None => None,
      FnValMaybe::FloatValue(value) => {
        self.radius_fn_ptr = Rc::new(move |_: CircleInArg| *value);
        None
      }
      FnValMaybe::FloatFn(radius_fn_ptr) => {
        self.radius_fn_ptr = radius_fn_ptr;
        None
      }
    };
  }

  fn precision(&mut self, precision: FnValMaybe) -> Option<f64> {
    match precision {
      FnValMaybe::None => None,
      FnValMaybe::FloatValue(value) => {
        self.precision_fn_ptr = Rc::new(move |_: CircleInArg| *value);
        None
      }
      FnValMaybe::FloatFn(precision_fn_ptr) => {
        self.precision_fn_ptr = precision_fn_ptr;
        None
      }
    }
  }
}
