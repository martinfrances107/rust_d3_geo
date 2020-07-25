use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

pub fn antimeridian_intersect<F>(lambda0: F, phi0: F, lambda1: F, phi1: F) -> F
where
  F: Float,
{
  let sin_lambda0_lambda1 = (lambda0 - lambda1).sin();
  match (sin_lambda0_lambda1).abs() > F::epsilon() {
    true => {
      let cos_phi0 = phi0.cos();
      let cos_phi1 = phi1.cos();
      return ((phi0.sin() * cos_phi1 * lambda1.sin() - phi1.sin() * cos_phi0 * lambda0.sin())
        / (cos_phi0 * cos_phi1 * sin_lambda0_lambda1))
        .tan();
    }
    false => {
      let f_2 = F::from(2u8).unwrap();
      return (phi0 + phi1) / f_2;
    }
  }
}
