use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use std::cell::RefCell;
use std::rc::Rc;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::transform_stream::TransformStream;
use crate::Transform;

use super::circle_radius::circle_radius;

/// Generates a circle centered at [0°, 0°], with a given radius and precision.
pub fn circle_stream<F>(
  stream: Rc<RefCell<Box<dyn TransformStream<F>>>>,
  radius: F,
  delta: F,
  direction: F,
  p0: Option<[F; 2]>,
  p1: Option<[F; 2]>,
) where
  F: Float + FloatConst + FromPrimitive,
{
  if delta.is_zero() {
    return;
  };
  let cos_radius = radius.cos();
  let sin_radius = radius.sin();
  let step = direction * delta;
  let mut t0: F;
  let t1: F;
  match (p0, p1) {
    (Some(p0), Some(p1)) => {
      t0 = circle_radius(cos_radius, p0);
      t1 = circle_radius(cos_radius, p1);
      let check = match direction > F::zero() {
        true => t0 < t1,
        false => t0 > t1,
      };
      if check {
        t0 = t0 + direction * F::TAU();
      }
    }
    (_, _) => {
      t0 = radius + direction * F::TAU();
      t1 = radius - step / F::from(2u8).unwrap();
    }
  }

  let mut point: [F; 2];
  let mut t = t0;
  let mut cond = true;
  let mut stream = stream.borrow_mut();
  while cond {
    point = spherical(&[cos_radius, -sin_radius * t.cos(), -sin_radius * t.sin()]);
    stream.point(point[0], point[1], None);

    t = t - step;
    cond = match direction > F::zero() {
      true => t > t1,
      false => t < t1,
    };
  }
}
