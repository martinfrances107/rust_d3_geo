use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::stream::GeoStream;

fn antimeridian_interpolate<F>(
  from: Option<[F; 2]>,
  to: [F; 2],
  direction: F,
  stream: &mut Box<dyn GeoStream<F>>,
) where
  F: Float + FloatConst,
{
  let phi: F;
  match from {
    None => {
      // phi = direction * HALFPI;
      phi = direction * F::FRAC_PI_2();
      stream.point(-F::PI(), phi);
      stream.point(F::zero(), phi);
      stream.point(F::PI(), phi);
      stream.point(F::PI(), F::zero());
      stream.point(F::PI(), -phi);
      stream.point(F::zero(), -phi);
      stream.point(-F::PI(), -phi);
      stream.point(-F::PI(), F::zero());
      stream.point(-F::PI(), phi);
    }
    Some(from) => match (from[0] - to[0]).abs() > F::epsilon() {
      true => {
        let lambda: F = if from[0] < to[0] { F::PI() } else { -F::PI() };
        let f_2 = F::from(2u8).unwrap();
        phi = direction * lambda / f_2;
        stream.point(-lambda, phi);
        stream.point(F::zero(), phi);
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
