use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;
// use super::adder::Adder;
use crate::cartesian::cartesian;
use crate::cartesian::cartesian_cross;
use crate::cartesian::cartesian_normalize_in_place;

// import adder from "./adder.js";
// var sum = adder();

fn longitude<F>(point: &[F; 2]) -> F
where
  F: Float + FloatConst
{
  if point[0].abs() <= F::PI() {
    return point[0];
  } else {
    return point[0].signum() * ((point[0].abs() + F::PI()) % F::TAU() - F::PI());
  }
}

pub fn contains<F>(polygon: Vec<Vec<[F; 2]>>, point: &[F; 2]) -> bool
where
  F: Float + FloatConst + FromPrimitive
{
  let lambda = longitude(point);
  let mut phi = point[1];
  let sin_phi = phi.sin();
  let normal = [lambda.sin(), -lambda.cos(), F::zero()];
  let mut angle = F::zero();
  // let sum = Adder::<F>::new();
  let mut sum = F::zero();
  let mut winding = 0i32;

  // New then reset is this needed.
  // sum.reset();

  if sin_phi == F::one() {
    phi = F::FRAC_PI_2() + F::epsilon();
  } else if sin_phi == -F::one() {
    phi = -F::FRAC_PI_2() - F::epsilon();
  }

  for polygon_i in polygon {
    let ring = polygon_i;
    let m = ring.len();
    if ring.is_empty() {
      continue;
    };

    let mut point0 = *ring.last().unwrap();
    let mut lambda0 = longitude(&point0);
    let phi0 = point0[1] / F::from(2u8).unwrap() + F::FRAC_PI_4();
    let mut sin_phi0 = phi0.sin();
    let mut cos_phi0 = phi0.cos();

    for j in 0..m {
      let point1 = ring[j];
      let lambda1 = longitude(&point1);
      let phi1 = point1[1] / F::from(2u8).unwrap() + F::FRAC_PI_4();
      let sin_phi1 = phi1.sin();
      let cos_phi1 = phi1.cos();
      let delta = lambda1 - lambda0;
      let sign = delta.signum();
      let abs_delta = sign * delta;
      let antimeridian = abs_delta > F::PI();
      let k = sin_phi0 * sin_phi1;

      // sum.add(atan2(k * sign * sin(absDelta), cosPhi0 * cosPhi1 + k * cos(absDelta)));
      sum = sum + (k * sign * abs_delta.sin()).atan2(cos_phi0 * cos_phi1 + k * abs_delta.cos());
      angle = angle
        + match antimeridian {
          true => delta + sign * F::TAU(),
          false => delta,
        };

      // Are the longitudes either side of the point’s meridian (lambda),
      // and are the latitudes smaller than the parallel (phi)?
      // if antimeridian ^ lambda0 >= lambda ^ lambda1 >= lambda {
      // if (antimeridian ^ lambda0 >= lambda ^ lambda1 >= lambda) {
      if antimeridian ^ (lambda0 >= lambda) ^ (lambda1 >= lambda) {
        let mut arc = cartesian_cross(&cartesian(&point0), &cartesian(&point1));
        cartesian_normalize_in_place(&mut arc);
        let mut intersection = cartesian_cross(&normal, &arc);
        cartesian_normalize_in_place(&mut intersection);
        let phi_arc: F;
        if antimeridian ^ (delta >= F::zero()) {
          phi_arc = -(intersection[2].asin());
        } else {
          phi_arc = intersection[2].asin();
        }

        if phi > phi_arc || phi == phi_arc && (!arc[0].is_zero() || !arc[1].is_zero()) {
          match antimeridian ^ (delta >= F::zero()) {
            true => winding = winding + 1,
            false => winding = winding - 1,
          };
        }
      }

      // Loop is about the restart.
      lambda0 = lambda1;
      sin_phi0 = sin_phi1;
      cos_phi0 = cos_phi1;
      point0 = point1;
    }
  }

  // First, determine whether the South pole is inside or outside:
  //
  // It is inside if:
  // * the polygon winds around it in a clockwise direction.
  // * the polygon does not (cumulatively) wind around it, but has a negative
  //   (counter-clockwise) area.
  //
  // Second, count the (signed) number of times a segment crosses a lambda
  // from the point to the South pole.  If it is zero, then the point is the
  // same side as the South pole.
  let is_winding_odd;
  if winding & 1 == 1 {
    is_winding_odd = true;
  } else {
    is_winding_odd = false;
  }

  let epsilon = F::from(1e-6).unwrap();
  let is_south_pole_inside = angle < -epsilon || angle < epsilon && sum < -epsilon;
  let ret = is_south_pole_inside ^ is_winding_odd;

  return ret;
}
