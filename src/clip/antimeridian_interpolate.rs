use std::f64::consts::PI;

use crate::math::EPSILON;
use crate::math::HALFPI;

use crate::projection::geo_stream::GeoStream;

fn antimeridian_interpolate(
  from: Option<[f64; 2]>,
  to: [f64; 2],
  direction: f64,
  stream: &mut Box<dyn GeoStream>,
) {
  let phi: f64;
  match from {
    None => {
      phi = direction * HALFPI;
      stream.point(-PI, phi);
      stream.point(0f64, phi);
      stream.point(PI, phi);
      stream.point(PI, 0f64);
      stream.point(PI, -phi);
      stream.point(0f64, -phi);
      stream.point(-PI, -phi);
      stream.point(-PI, 0f64);
      stream.point(-PI, phi);
    }
    Some(from) => match (from[0] - to[0]).abs() > EPSILON {
      true => {
        let lambda: f64 = if from[0] < to[0] { PI } else { -PI };
        phi = direction * lambda / 2f64;
        stream.point(-lambda, phi);
        stream.point(0f64, phi);
        stream.point(lambda, phi);
      }
      false => {
        stream.point(to[0], to[1]);
      }
    },
  }
}

// function clipAntimeridianInterpolate(from, to, direction, stream) {
//   var phi;
//   if (from == null) {
//     phi = direction * halfPI;
//     stream.point(-PI, phi);
//     stream.point(0, phi);
//     stream.point(PI, phi);
//     stream.point(PI, 0);
//     stream.point(PI, -phi);
//     stream.point(0, -phi);
//     stream.point(-PI, -phi);
//     stream.point(-PI, 0);
//     stream.point(-PI, phi);
//   } else if (abs(from[0] - to[0]) > EPSILON) {
//     var lambda = from[0] < to[0] ? PI : -PI;
//     phi = direction * lambda / 2;
//     stream.point(-lambda, phi);
//     stream.point(0, phi);
//     stream.point(lambda, phi);
//   } else {
//     stream.point(to[0], to[1]);
//   }
// }
