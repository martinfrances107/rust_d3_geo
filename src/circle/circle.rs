use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use num_traits::cast::cast;
use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

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
pub struct CircleStream<F> {
  stream_type: StreamType,
  pub coordinates: Vec<Vec<[F; 2]>>,
}

#[derive(Clone)]
pub struct Circle<F>
where
  F: Float,
{
  center_fn_ptr: Rc<dyn Fn(CircleInArg) -> [F; 2]>,
  radius_fn_ptr: Rc<dyn Fn(CircleInArg) -> F>,
  precision_fn_ptr: Rc<dyn Fn(CircleInArg) -> F>,
}

fn center<F>(_in: CircleInArg) -> [F; 2]
where
  F: Float + FloatConst,
{
  return [F::zero(), F::zero()];
}

fn radius<F>(_in: CircleInArg) -> F
where
  F: Float + FloatConst,
{
  return F::from(90u8).unwrap();
}

fn precision<F>(_in: CircleInArg) -> F
where
  F: Float + FloatConst,
{
  return F::from(6u8).unwrap();
}

impl<F> Circle<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  pub fn new() -> Self {
    let center_fn_ptr = Rc::new(center);
    let radius_fn_ptr = Rc::new(radius);
    let precision_fn_ptr = Rc::new(precision);

    let c_val: [F; 2] = (*center_fn_ptr)(CircleInArg::None);

    return Self {
      center_fn_ptr,
      radius_fn_ptr,
      precision_fn_ptr,
    };
  }

  pub fn circle(&mut self, _arg: CircleInArg) -> CircleStream<F> {
    // TODO must come back and copy the arg so in can be passes into each fn c, r and p.
    let c = (*self.center_fn_ptr)(CircleInArg::None);
    let r = (*self.radius_fn_ptr)(CircleInArg::None).to_radians();
    let p = (*self.precision_fn_ptr)(CircleInArg::None).to_radians();

    let ring = Rc::new(RefCell::new(Vec::new()));

    let rotate = Rc::new(RotateRadians::new(
      -c[0].to_radians(),
      -c[1].to_radians(),
      F::zero(),
    ));

    let stream = Rc::new(RefCell::new(Stream::new(rotate.clone(), ring.clone())));

    circle_stream(stream, r, p, F::one(), None, None);

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

impl<F> CircleTrait<F> for Circle<F>
where
  F: Float + 'static,
{
  fn center(&mut self, center: FnValMaybe2D<F>) -> Option<[F; 2]> {
    return match center {
      FnValMaybe2D::None => None,
      FnValMaybe2D::FloatValue(value) => {
        self.center_fn_ptr = Rc::new(move |_: CircleInArg| *value);
        None
      }
      FnValMaybe2D::FloatFn(center_fn_ptr) => {
        self.center_fn_ptr = center_fn_ptr;
        None
      }
    };
  }

  fn radius(&mut self, radius: FnValMaybe<F>) -> Option<F> {
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

  fn precision(&mut self, precision: FnValMaybe<F>) -> Option<F> {
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
