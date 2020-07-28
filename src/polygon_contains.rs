#[allow(unused_imports)]

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use super::adder::Adder;
use crate::cartesian::cartesian;
use crate::cartesian::cartesian_cross;
use crate::cartesian::cartesian_normalize_in_place;

// import adder from "./adder.js";


// var sum = adder();

// function longitude(point) {
//   if (abs(point[0]) <= PI)
//     return point[0];
//   else
//     return sign(point[0]) * ((abs(point[0]) + PI) % TAU - PI);
// }

fn longitude<F>(point: &[F;2]) -> F
where F: Float + FloatConst {
  if point[0].abs() <= F::PI() {
    return point[0];
  }
  else {
    return point[0].signum() * ( (point[0] + F::PI()).abs() % F::TAU()-F::PI());
  }
}

fn contains<F>(polygon: Vec<Vec<[F;2]>>, point: &[F;2]) -> bool
where F: Float + FloatConst + FromPrimitive {
  let lambda = longitude(point);
  let phi = point[1];
  let sinPhi = phi.sin();
  let normal = [lambda.sin(), -lambda.cos(), F::zero()];
  let angle = F::zero();
  let sum = Adder::<F>::new();
  let winding = 0i32;

  // New then reset is this needed.
  sum.reset();

  if sinPhi == F::one() {
    phi = F::FRAC_PI_2() + F::epsilon();
  }
  else if sinPhi == -F::one() {
    phi = -F::FRAC_PI_2() - F::epsilon();
  }

  // for (var i = 0, n = polygon.len(); i < n; ++i) {
  for polygon_i in polygon.iter() {
    let ring;
    let m;
    // if (!(m = (ring = polygon).len())) continue;

    ring = polygon_i;
    m = ring.len();
    if polygon.is_empty() {continue};

    let point0 = ring.last().unwrap();
    let point1;
    let lambda0 = longitude(&point0);
    let lambda1;
    let phi0 = point0[1] / F::from(2u8).unwrap() + F::FRAC_PI_4();
    let phi1;
    let sinPhi0 = phi0.sin();
    let sinPhi1;
    let cosPhi0 = phi0.cos();
    let cosPhi1;

    // for (var j = 0; j < m; ++j, lambda0 = lambda1, sinPhi0 = sinPhi1, cosPhi0 = cosPhi1, point0 = point1) {
    for j in 0..m {
      let point1 = ring[j];
      let lambda1 = longitude(&point1);
      let phi1 = point1[1] / F::from(2u8).unwrap() + F::FRAC_PI_4();
      let sinPhi1 = phi1.sin();
      let cosPhi1 = phi1.cos();
      let delta = lambda1 - lambda0;
      // let sign = delta >= 0 ? 1 : -1;
      let sign = delta.signum();
      let absDelta = sign * delta;
      let antimeridian = absDelta > F::PI();
      let k = sinPhi0 * sinPhi1;

      sum.add(
        (k * sign * absDelta.sin()).atan2(cosPhi0 * cosPhi1 + k * absDelta.cos()));
      // angle += antimeridian ? delta + sign * TAU : delta;
      angle = angle + match antimeridian {
        true => { delta + sign * F::TAU() },
        false => { delta },
      };

      // Are the longitudes either side of the point’s meridian (lambda),
      // and are the latitudes smaller than the parallel (phi)?
      // if antimeridian ^ lambda0 >= lambda ^ lambda1 >= lambda {
      if antimeridian ^ lambda0 >= lambda ^ lambda1 &&
         lambda ^ lambda1 >= lambda {
        let arc = cartesian_cross(&cartesian(&point0), &cartesian(&point1));
        cartesian_normalize_in_place(&mut arc);
        let intersection = cartesian_cross(&normal, &arc);
        cartesian_normalize_in_place(&mut intersection);

//         var phiArc = (antimeridian ^ delta >= 0 ? -1 : 1) * asin(intersection[2]);
        let phiArc = match antimeridian ^ delta >= 0 {
          true =>  {  intersection[2].asin()},
          false => { -intersection[2].asin()}
        };


//         if (phi > phiArc || phi === phiArc && (arc[0] || arc[1])) {
//           winding += antimeridian ^ delta >= 0 ? 1 : -1;
//         }


        if phi > phiArc || phi == phiArc && (!arc[0].is_zero() || !arc[1].is_zero()) {
           match  delta >= F::zero() {
            true => { winding = winding + 1 },
            false => { winding = winding -1 },
          };

        }
      }

    }

    // loop is about the restart
    lambda0 = lambda1;
    sinPhi0 = sinPhi1;
    cosPhi0 = cosPhi1;
    point0 = point1;

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

  return
    (angle < -F::epsilon() ||
    angle <F::epsilon() &&
    sum < -F::epsilon()) ^ (winding & 1);
}


// use std::f64::PI;
// use crate::math::HALFPI;
// use crate::math::QUATERPI;
// use crate::math::EPSILON;
// use crate::math::TAU;


// // import adder from "./adder.js";
// // import {cartesian, cartesianCross, cartesianNormalizeInPlace} from "./cartesian.js";
// // import {abs, asin, atan2, cos, epsilon, halfPI, PI, quarterPI, sign, sin, TAU} from "./math.js";

// var sum = adder();

// function longitude(point) {
//   if (abs(point[0]) <= PI)
//     return point[0];
//   else
//     return sign(point[0]) * ((abs(point[0]) + PI) % TAU - PI);
// }

// export default function(polygon, point) {
//   var lambda = longitude(point),
//       phi = point[1],
//       sinPhi = sin(phi),
//       normal = [sin(lambda), -cos(lambda), 0],
//       angle = 0,
//       winding = 0;

//   sum.reset();

//   if (sinPhi === 1) phi = halfPI + epsilon;
//   else if (sinPhi === -1) phi = -halfPI - epsilon;

//   for (var i = 0, n = polygon.len(); i < n; ++i) {
//     if (!(m = (ring = polygon[i]).len())) continue;
//     var ring,
//         m,
//         point0 = ring[m - 1],
//         lambda0 = longitude(point0),
//         phi0 = point0[1] / 2 + quarterPI,
//         sinPhi0 = sin(phi0),
//         cosPhi0 = cos(phi0);

//     for (var j = 0; j < m; ++j, lambda0 = lambda1, sinPhi0 = sinPhi1, cosPhi0 = cosPhi1, point0 = point1) {
//       var point1 = ring[j],
//           lambda1 = longitude(point1),
//           phi1 = point1[1] / 2 + quarterPI,
//           sinPhi1 = sin(phi1),
//           cosPhi1 = cos(phi1),
//           delta = lambda1 - lambda0,
//           sign = delta >= 0 ? 1 : -1,
//           absDelta = sign * delta,
//           antimeridian = absDelta > PI,
//           k = sinPhi0 * sinPhi1;

//       sum.add(atan2(k * sign * sin(absDelta), cosPhi0 * cosPhi1 + k * cos(absDelta)));
//       angle += antimeridian ? delta + sign * TAU : delta;

//       // Are the longitudes either side of the point’s meridian (lambda),
//       // and are the latitudes smaller than the parallel (phi)?
//       if (antimeridian ^ lambda0 >= lambda ^ lambda1 >= lambda) {
//         var arc = cartesianCross(cartesian(point0), cartesian(point1));
//         cartesianNormalizeInPlace(arc);
//         var intersection = cartesianCross(normal, arc);
//         cartesianNormalizeInPlace(intersection);
//         var phiArc = (antimeridian ^ delta >= 0 ? -1 : 1) * asin(intersection[2]);
//         if (phi > phiArc || phi === phiArc && (arc[0] || arc[1])) {
//           winding += antimeridian ^ delta >= 0 ? 1 : -1;
//         }
//       }
//     }
//   }

//   // First, determine whether the South pole is inside or outside:
//   //
//   // It is inside if:
//   // * the polygon winds around it in a clockwise direction.
//   // * the polygon does not (cumulatively) wind around it, but has a negative
//   //   (counter-clockwise) area.
//   //
//   // Second, count the (signed) number of times a segment crosses a lambda
//   // from the point to the South pole.  If it is zero, then the point is the
//   // same side as the South pole.

//   return (angle < -F::epsion()|| angle <F::epsion()&& sum < -epsilon) ^ (winding & 1);
// }

#[cfg(test)]
mod tests {
  use super::*;

  // import geoCircle from "../src/circle.js";

  fn polygon_contains<F>(polygon: Vec::<[F;2]>, point: &[F;2]) -> bool
  where F: Float + FloatConst + FromPrimitive {
    return contains(polygon.map(ring_radians), &point_radians(point));
  }

  #[test]
  fn empty_return_false() {
    println!("geoPolygonContains(empty, point) returns false");
    assert!(polygon_contains(Vec::new(), &[0f64, 0f64]), 0f64);
  }

// tape("geoPolygonContains(simple, point) returns the expected value", function(test) {
//   var polygon = [[[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]]];
//   test.equal(polygonContains(polygon, [0.1, 2]), 0);
//   test.equal(polygonContains(polygon, [0.1, 0.1]), 1);
//   test.end();
// });

// tape("geoPolygonContains(smallCircle, point) returns the expected value", function(test) {
//   var polygon = geoCircle().radius(60)().coordinates;
//   test.equal(polygonContains(polygon, [-180, 0]), 0);
//   test.equal(polygonContains(polygon, [1, 1]), 1);
//   test.end();
// });

// tape("geoPolygonContains wraps longitudes", function(test) {
//   var polygon = geoCircle().center([300, 0])().coordinates;
//   test.equal(polygonContains(polygon, [300, 0]), 1);
//   test.equal(polygonContains(polygon, [-60, 0]), 1);
//   test.equal(polygonContains(polygon, [-420, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(southPole, point) returns the expected value", function(test) {
//   var polygon = [[[-60, -80], [60, -80], [180, -80], [-60, -80]]];
//   test.equal(polygonContains(polygon, [0, 0]), 0);
//   test.equal(polygonContains(polygon, [0, -85]), 1);
//   test.equal(polygonContains(polygon, [0, -90]), 1);
//   test.end();
// });

// tape("geoPolygonContains(northPole, point) returns the expected value", function(test) {
//   var polygon = [[[60, 80], [-60, 80], [-180, 80], [60, 80]]];
//   test.equal(polygonContains(polygon, [0, 0]), 0);
//   test.equal(polygonContains(polygon, [0, 85]), 1);
//   test.equal(polygonContains(polygon, [0, 90]), 1);
//   test.equal(polygonContains(polygon, [-100, 90]), 1);
//   test.equal(polygonContains(polygon, [0, -90]), 0);
//   test.end();
// });

// tape("geoPolygonContains(touchingPole, Pole) returns true (issue #105)", function(test) {
//   var polygon = [[[0, -30], [120, -30], [0, -90], [0, -30]]];
//   test.equal(polygonContains(polygon, [0, -90]), 0);
//   test.equal(polygonContains(polygon, [-60, -90]), 0);
//   test.equal(polygonContains(polygon, [60, -90]), 0);
//   polygon = [[[0, 30], [-120, 30], [0, 90], [0, 30]]];
//   test.equal(polygonContains(polygon, [0, 90]), 0);
//   test.equal(polygonContains(polygon, [-60, 90]), 0);
//   test.equal(polygonContains(polygon, [60, 90]), 0);
//   test.end();
// });

// tape("geoPolygonContains(southHemispherePoly) returns the expected value", function(test) {
//   var polygon = [[[0, 0], [10, -40], [-10, -40], [0, 0]]];
//   test.equal(polygonContains(polygon, [0,-40.2]), 1);
//   test.equal(polygonContains(polygon, [0,-40.5]), 0);
//   test.end();
// });

// tape("geoPolygonContains(largeNearOrigin, point) returns the expected value", function(test) {
//   var polygon = [[[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]];
//   test.equal(polygonContains(polygon, [0.1, 0.1]), 0);
//   test.equal(polygonContains(polygon, [2, 0.1]), 1);
//   test.end();
// });

// tape("geoPolygonContains(largeNearSouthPole, point) returns the expected value", function(test) {
//   var polygon = [[[-60, 80], [60, 80], [180, 80], [-60, 80]]];
//   test.equal(polygonContains(polygon, [0, 85]), 0);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(largeNearNorthPole, point) returns the expected value", function(test) {
//   var polygon = [[[60, -80], [-60, -80], [-180, -80], [60, -80]]];
//   test.equal(polygonContains(polygon, [0, -85]), 0);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(largeCircle, point) returns the expected value", function(test) {
//   var polygon = geoCircle().radius(120)().coordinates;
//   test.equal(polygonContains(polygon, [-180, 0]), 0);
//   test.equal(polygonContains(polygon, [-90, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(largeNarrowStripHole, point) returns the expected value", function(test) {
//   var polygon = [[[-170, -1], [0, -1], [170, -1], [170, 1], [0, 1], [-170, 1], [-170, -1]]];
//   test.equal(polygonContains(polygon, [0, 0]), 0);
//   test.equal(polygonContains(polygon, [0, 20]), 1);
//   test.end();
// });

// tape("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value", function(test) {
//   var circle = geoCircle().center([0, -90]),
//       ring0 = circle.radius(90 - 0.01)().coordinates[0],
//       ring1 = circle.radius(90 + 0.01)().coordinates[0].reverse(),
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0, 0]), 0);
//   test.equal(polygonContains(polygon, [0, -90]), 1);
//   test.end();
// });

fn large_narrow_equatorialHole() {
  println!("geoPolygonContains(empty, point) returns false");
  assert!(polygon_contains(Vec::new(), &[0f64, 0f64]), 0f64);
}


// tape("geoPolygonContains(largeNarrowEquatorialStrip, point) returns the expected value", function(test) {
//   var circle = geoCircle().center([0, -90]),
//       ring0 = circle.radius(90 + 0.01)().coordinates[0],
//       ring1 = circle.radius(90 - 0.01)().coordinates[0].reverse(),
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0, -90]), 0);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(ringNearOrigin, point) returns the expected value", function(test) {
//   var ring0 = [[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]],
//       ring1 = [[0.4, 0.4], [0.6, 0.4], [0.6, 0.6], [0.4, 0.6], [0.4, 0.4]],
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0.5, 0.5]), 0);
//   test.equal(polygonContains(polygon, [0.1, 0.5]), 1);
//   test.end();
// });

// tape("geoPolygonContains(ringEquatorial, point) returns the expected value", function(test) {
//   var ring0 = [[0, -10], [-120, -10], [120, -10], [0, -10]],
//       ring1 = [[0, 10], [120, 10], [-120, 10], [0, 10]],
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0, 20]), 0);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value", function(test) {
//   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]].reverse(),
//       ring1 = [[170, 10], [170, -10], [-170, -10], [-170, 10], [170, 10]].reverse(),
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0, 90]), 0);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(ringContainingBothPoles, point) returns the expected value", function(test) {
//   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]],
//       ring1 = [[170, 10], [170, -10], [-170, -10], [-170, 10], [170, 10]],
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0, 0]), 0);
//   test.equal(polygonContains(polygon, [0, 20]), 1);
//   test.end();
// });

// tape("geoPolygonContains(ringContainingSouthPole, point) returns the expected value", function(test) {
//   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]],
//       ring1 = [[0, 80], [120, 80], [-120, 80], [0, 80]],
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0, 90]), 0);
//   test.equal(polygonContains(polygon, [0, -90]), 1);
//   test.end();
// });

// tape("geoPolygonContains(ringContainingNorthPole, point) returns the expected value", function(test) {
//   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]].reverse(),
//       ring1 = [[0, 80], [120, 80], [-120, 80], [0, 80]].reverse(),
//       polygon = [ring0, ring1];
//   test.equal(polygonContains(polygon, [0, -90]), 0);
//   test.equal(polygonContains(polygon, [0, 90]), 1);
//   test.end();
// });

// tape("geoPolygonContains(selfIntersectingNearOrigin, point) returns the expected value", function(test) {
//   var polygon = [[[0, 0], [1, 0], [1, 3], [3, 3], [3, 1], [0, 1], [0, 0]]];
//   test.equal(polygonContains(polygon, [15, 0.5]), 0);
//   test.equal(polygonContains(polygon, [12, 2]), 0);
//   test.equal(polygonContains(polygon, [0.5, 0.5]), 1);
//   test.equal(polygonContains(polygon, [2, 2]), 1);
//   test.end();
// });

// tape("geoPolygonContains(selfIntersectingNearSouthPole, point) returns the expected value", function(test) {
//   var polygon = [[[-10, -80], [120, -80], [-120, -80], [10, -85], [10, -75], [-10, -75], [-10, -80]]];
//   test.equal(polygonContains(polygon, [0, 0]), 0);
//   test.equal(polygonContains(polygon, [0, -76]), 1);
//   test.equal(polygonContains(polygon, [0, -89]), 1);
//   test.end();
// });

// tape("geoPolygonContains(selfIntersectingNearNorthPole, point) returns the expected value", function(test) {
//   var polygon = [[[-10, 80], [-10, 75], [10, 75], [10, 85], [-120, 80], [120, 80], [-10, 80]]];
//   test.equal(polygonContains(polygon, [0, 0]), 0);
//   test.equal(polygonContains(polygon, [0, 76]), 1);
//   test.equal(polygonContains(polygon, [0, 89]), 1);
//   test.end();
// });

// tape("geoPolygonContains(hemisphereTouchingTheSouthPole, point) returns the expected value", function(test) {
//   var polygon = geoCircle().radius(90)().coordinates;
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.end();
// });

// tape("geoPolygonContains(triangleTouchingTheSouthPole, point) returns the expected value", function(test) {
//   var polygon = [[[180, -90], [-45, 0], [45, 0], [180, -90]]];
//   test.equal(polygonContains(polygon, [-46, 0]), 0);
//   test.equal(polygonContains(polygon, [0, 1]), 0);
//   test.equal(polygonContains(polygon, [-90, -80]), 0);
//   test.equal(polygonContains(polygon, [-44, 0]), 1);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.equal(polygonContains(polygon, [0, -30]), 1);
//   test.equal(polygonContains(polygon, [30, -80]), 1);
//   test.end();
// });

// tape("geoPolygonContains(triangleTouchingTheSouthPole2, point) returns the expected value", function(test) {
//   var polygon = [[[-45, 0], [45, 0], [180, -90], [-45, 0]]];
//   test.equal(polygonContains(polygon, [-46, 0]), 0);
//   test.equal(polygonContains(polygon, [0, 1]), 0);
//   test.equal(polygonContains(polygon, [-90, -80]), 0);
//   test.equal(polygonContains(polygon, [-44, 0]), 1);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.equal(polygonContains(polygon, [0, -30]), 1);
//   test.equal(polygonContains(polygon, [30, -80]), 1);
//   test.end();
// });

// tape("geoPolygonContains(triangleTouchingTheSouthPole3, point) returns the expected value", function(test) {
//   var polygon = [[[180, -90], [-135, 0], [135, 0], [180, -90]]];
//   test.equal(polygonContains(polygon, [180, 0]), 0);
//   test.equal(polygonContains(polygon, [150, 0]), 0);
//   test.equal(polygonContains(polygon, [180, -30]), 0);
//   test.equal(polygonContains(polygon, [150, -80]), 0);
//   test.equal(polygonContains(polygon, [0, 0]), 1);
//   test.equal(polygonContains(polygon, [180, 1]), 1);
//   test.equal(polygonContains(polygon, [-90, -80]), 1);
//   test.end();
// });

// tape("geoPolygonContains(triangleTouchingTheNorthPole, point) returns the expected value", function(test) {
//   var polygon = [[[180, 90], [45, 0], [-45, 0], [180, 90]]];
//   test.equal(polygonContains(polygon, [-90, 0]), 0);
//   test.equal(polygonContains(polygon, [0, -1]), 0);
//   test.equal(polygonContains(polygon, [0, -80]), 0);
//   test.equal(polygonContains(polygon, [-90, 1]), 0);
//   test.equal(polygonContains(polygon, [-90, 80]), 0);
//   test.equal(polygonContains(polygon, [-44, 10]), 1);
//   test.equal(polygonContains(polygon, [0, 10]), 1);
//   test.equal(polygonContains(polygon, [30, 80]), 1);
//   test.end();
// });

// function ringRadians(ring) {
//   return ring = ring.map(pointRadians), ring.pop(), ring;
// }

fn ring_radians<F>(ring: Vec::<[F;2]>) -> Vec::<[F;2]>
where F: Float {
  ring.iter().map(point_radians);
  ring.pop();
  return ring;
}

// function pointRadians(point) {
//   return [point[0] * Math.PI / 180, point[1] * Math.PI / 180];
// }

fn point_radians<F>(point: &[F;2]) -> [F;2]
where F: Float {
  return [point[0].to_radians(), point[1].to_radians()];
}

}