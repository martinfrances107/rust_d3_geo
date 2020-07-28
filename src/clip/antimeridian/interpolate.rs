use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::stream::GeoStream;

pub fn interpolate<F>(
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
