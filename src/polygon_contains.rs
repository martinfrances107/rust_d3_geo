use delaunator::Point;
use std::f64;
// use super::adder::Adder;
use crate::cartesian::cartesian;
use crate::cartesian::cartesian_cross;
use crate::cartesian::cartesian_normalize_in_place;
use crate::math::EPSILON;
use crate::math::TAU;

// import adder from "./adder.js";
// var sum = adder();

fn longitude(point: &Point) -> f64 {
    if point.x.abs() <= f64::consts::PI {
        return point.x;
    } else {
        return point.x.signum() * ((point.x.abs() + f64::consts::PI) % TAU - f64::consts::PI);
    }
}

pub fn contains(polygon: Vec<Vec<Point>>, point: &Point) -> bool {
    let lambda = longitude(point);
    let mut phi = point.y;
    let sin_phi = phi.sin();
    let normal = [lambda.sin(), -lambda.cos(), 0f64];
    let mut angle = 0f64;
    // let sum = Adder::<F>::new();
    let mut sum = 0f64;
    let mut winding = 0i32;

    // New then reset is this needed.
    // sum.reset();

    if sin_phi == 1f64 {
        phi = f64::consts::FRAC_PI_2 + f64::EPSILON;
    } else if sin_phi == -1f64 {
        phi = -f64::consts::FRAC_PI_2 - f64::EPSILON;
    }

    for polygon_i in polygon {
        let ring = polygon_i;
        let m = ring.len();
        if ring.is_empty() {
            continue;
        };

        let mut point0 = (*ring.last().unwrap()).clone();
        let mut lambda0 = longitude(&point0);
        let phi0 = point0.y / 2f64 + f64::consts::FRAC_PI_4;
        let mut sin_phi0 = phi0.sin();
        let mut cos_phi0 = phi0.cos();

        for j in 0..m {
            let point1 = ring[j].clone();
            let lambda1 = longitude(&point1);
            let phi1 = point1.y / 2f64 + f64::consts::FRAC_PI_4;
            let sin_phi1 = phi1.sin();
            let cos_phi1 = phi1.cos();
            let delta = lambda1 - lambda0;
            let sign = delta.signum();
            let abs_delta = sign * delta;
            let antimeridian = abs_delta > f64::consts::PI;
            let k = sin_phi0 * sin_phi1;

            // sum.add(atan2(k * sign * sin(absDelta), cosPhi0 * cosPhi1 + k * cos(absDelta)));
            sum =
                sum + (k * sign * abs_delta.sin()).atan2(cos_phi0 * cos_phi1 + k * abs_delta.cos());
            angle = angle
                + match antimeridian {
                    true => delta + sign * TAU,
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
                let phi_arc: f64;
                if antimeridian ^ (delta >= 0f64) {
                    phi_arc = -(intersection[2].asin());
                } else {
                    phi_arc = intersection[2].asin();
                }

                if phi > phi_arc || phi == phi_arc && (arc[0] != 0f64 || arc[1] != 0f64) {
                    match antimeridian ^ (delta >= 0f64) {
                        true => winding += 1,
                        false => winding -= 1,
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

    let is_south_pole_inside = angle < -EPSILON || angle < EPSILON && sum < -EPSILON;
    let ret = is_south_pole_inside ^ is_winding_odd;

    return ret;
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

//   return (angle < -epsilon || angle < epsilon && sum < -epsilon) ^ (winding & 1);
// }
