#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::Debug;
use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::Transform;

use crate::rotation::rotate_radians::rotate_radians;
use crate::stream::GeoStream;

/// Returns the signed angle of a cartesian point relative to [cosRadius, 0, 0].
fn circle_radius<F>(cos_radius: F, point_p: [F; 2]) -> F
where
  F: Float + FloatConst + FromPrimitive,
{
  let mut point = cartesian(&point_p);
  point[0] = point[0] - cos_radius;
  cartesian_normalize_in_place(&mut point);
  let radius = (-point[1]).acos();
  let radius_signed = match -point[2] < F::zero() {
    true => -radius,
    false => radius,
  };
  return (radius_signed + F::TAU() - F::epsilon()) % F::TAU();
}

pub struct Circle<F> {
  center: [F; 2],
  radius: F,
  precision: F,
  ring: Vec<[F; 2]>,
  pub coordinates: Vec<Vec<[F; 2]>>,
  rotate: Box<dyn Transform<F>>,
}

impl<F: 'static> Circle<F>
where
  F: Float + FloatConst + FromPrimitive + Debug,
{
  pub fn new(center_p: Option<[F; 2]>, radius_p: Option<F>, precision_p: Option<F>) -> Self {
    let center = center_p.unwrap_or([F::zero(), F::zero()]);
    let mut radius = radius_p.unwrap_or(F::from(90u8).unwrap());
    radius = radius.to_radians();
    let mut precision = precision_p.unwrap_or(F::from(6u8).unwrap());
    precision = precision.to_radians();

    let ring = Vec::new();
    let rotate = rotate_radians(-center[0].to_radians(), -center[1].to_radians(), F::zero());
    let coordinates = Vec::new();
    // c = {type: "Polygon", coordinates: [ring]};
    // ring = rotate = null;
    let mut c = Self {
      center,
      coordinates,
      precision,
      radius,
      ring,
      rotate,
    };

    c.circle_stream(radius, precision, F::one(), None, None);
    c.coordinates.push(c.ring);
    c.ring = Vec::new();
    return c;
  }

  // function circle() {
  //   var c = center.apply(this, arguments),
  //       r = radius.apply(this, arguments) * radians,
  //       p = precision.apply(this, arguments) * radians;
  //   ring = [];
  //   rotate = rotateRadians(-c[0] * radians, -c[1] * radians, 0).invert;
  //   circleStream(stream, r, p, 1);
  //   c = {type: "Polygon", coordinates: [ring]};
  //   ring = rotate = null;
  //   return c;
  // }

  // circle.center = function(_) {
  //   return arguments.length ? (center = typeof _ === "function" ? _ : constant([+_[0], +_[1]]), circle) : center;
  // };

  /// Generates a circle centered at [0°, 0°], with a given radius and precision.
  fn circle_stream(
    &mut self,
    radius: F,
    delta: F,
    direction: F,
    p0: Option<[F; 2]>,
    p1: Option<[F; 2]>,
  ) where
    F: Float + FloatConst + FromPrimitive + Debug,
  {
    if delta.is_zero() {
      return;
    };
    let cos_radius = radius.cos();
    let sin_radius = radius.sin();
    let step = direction * delta;
    let mut t0: F;
    let t1: F;
    // println!("radius {:?}", radius);
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
        // println!("NULL");
        t0 = radius + direction * F::TAU();
        t1 = radius - step / F::from(2u8).unwrap();
      }
    }
    // if (t0 == null) {
    //   t0 = radius + direction * tau;
    //   t1 = radius - step / 2;
    // } else {
    //   t0 = circleRadius(cosRadius, t0);
    //   t1 = circleRadius(cosRadius, t1);
    //   if (direction > 0 ? t0 < t1 : t0 > t1) t0 += direction * tau;
    // }

    // for (var point, t = t0; direction > 0 ? t > t1 : t < t1; t -= step) {
    //   point = spherical([cosRadius, -sinRadius * cos(t), -sinRadius * sin(t)]);
    //   stream.point(point[0], point[1]);
    // }

    let mut point: [F; 2];
    let mut t = t0;
    let mut cond = true;
    while cond {
      // println!("circle enter for loop t = {:?}", t);
      point = spherical(&[cos_radius, -sin_radius * t.cos(), -sin_radius * t.sin()]);
      self.point(point[0], point[1]);

      t = t - step;
      cond = match direction > F::zero() {
        true => t > t1,
        false => t < t1,
      };
    }
  }

  pub fn get_center(&self) -> [F; 2] {
    return self.center;
  }

  pub fn center(&mut self, center: &[F; 2]) {
    self.center = *center;
  }

  pub fn get_radius(&self) -> F {
    return self.radius;
  }

  pub fn radius(&mut self, r: F) {
    self.radius = r;
  }

  // circle.precision = function(_) {
  //   return arguments.length ? (precision = typeof _ === "function" ? _ : constant(+_), circle) : precision;
  // };

  // return circle;
}

impl<F> GeoStream<F> for Circle<F>
where
  F: Float + Debug,
{
  fn point(&mut self, x: F, y: F)
  where
    F: Float,
  {
    let mut x_rotated = self.rotate.invert(&[x, y]);
    x_rotated[0] = x_rotated[0].to_degrees();
    x_rotated[1] = x_rotated[1].to_degrees();
    self.ring.push(x_rotated);
  }
}
