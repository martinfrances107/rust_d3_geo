use std::rc::Rc;

use num_traits::Float;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::Transform;
// use crate::TransformIdentity;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

pub struct RotateRadians {}

impl RotateRadians
{
  /// Returns a object implmenting the desired combination of rotations.
  pub fn new<F>(delta_lambda_p: F, delta_phi: F, delta_gamma: F) -> Box<dyn Transform<F>>
  where
    F: Float + FloatConst + 'static,
  {
    let delta_lambda = delta_lambda_p % F::TAU();
    // Should I rotate by lambda, phi or gamma.
    let by_lambda = !delta_lambda.is_zero();
    let by_phi = !delta_phi.is_zero();
    let by_gamma = !delta_gamma.is_zero();
    return match (by_lambda, by_gamma, by_phi) {
      (true, true, true) | (true, true, false) | (true, false, true) => Compose::new(
        Rc::new(RotationLambda::new(delta_lambda)),
        Rc::new(RotationPhiGamma::new(delta_phi, delta_gamma)),
      ),
      (true, false, false) => RotationLambda::new(delta_lambda),
      (false, true, true) | (false, true, false) | (false, false, true) => {
        RotationPhiGamma::new(delta_phi, delta_gamma)
      }
      (false, false, false) => RotationIdentity::new(),
    };
  }
}
