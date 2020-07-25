use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::stream::GeoStream;

fn antimeridian_interpolate<T>(
  from: Option<[T; 2]>,
  to: [T; 2],
  direction: T,
  stream: &mut Box<dyn GeoStream<T>>,
) where
  T: Float + FloatConst,
{
  let phi: T;
  match from {
    None => {
      // phi = direction * HALFPI;
      phi = direction * T::FRAC_PI_2();
      stream.point(-T::PI(), phi);
      stream.point(T::zero(), phi);
      stream.point(T::PI(), phi);
      stream.point(T::PI(), T::zero());
      stream.point(T::PI(), -phi);
      stream.point(T::zero(), -phi);
      stream.point(-T::PI(), -phi);
      stream.point(-T::PI(), T::zero());
      stream.point(-T::PI(), phi);
    }
    Some(from) => match (from[0] - to[0]).abs() > T::epsilon() {
      true => {
        let lambda: T = if from[0] < to[0] { T::PI() } else { -T::PI() };
        let f_2 = T::from(2u8).unwrap();
        phi = direction * lambda / f_2;
        stream.point(-lambda, phi);
        stream.point(T::zero(), phi);
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
