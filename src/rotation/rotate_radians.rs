use num_traits::Float;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::Transform;

use super::rotation_lambda::RotationLambda;
use super::rotation_identity::RotationIdentity;
use super::rotation_phi_gamma::RotationPhiGamma;

pub fn rotate_radians<F: 'static >(delta_lambda_p: F, delta_phi: F, delta_gamma: F) -> Box<dyn Transform<F>>
  where F: Float + FloatConst {
  let delta_lambda = delta_lambda_p % F::TAU();
  // Should I rotate by lambda, phi or gamma.
  let by_lambda = !delta_lambda.is_zero();
  let by_phi = !delta_phi.is_zero();
  let by_gamma = !delta_gamma.is_zero();

  return match (by_lambda, by_gamma, by_phi) {
    (true, true, true) | (true, true, false) | (true, false, true) => Box::new(Compose::new(
      Box::new(RotationLambda::new(delta_lambda)),
      Box::new(RotationPhiGamma::new(delta_phi, delta_gamma)),
    )),
    (true, false, false) =>  Box::new(RotationLambda::new(delta_lambda)),
    (false, true, true) | (false, true, false) | (false, false, true) => {
      Box::new(RotationPhiGamma::new(delta_phi, delta_gamma))
    }
    (false, false, false) => Box::new(RotationIdentity {}),
  };
}
