use std::f64;
use std::rc::Rc;

use crate::compose::Compose;
use crate::math::TAU;
use crate::Transform;
// use crate::TransformIdentity;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

pub struct RotateRadians {}

impl RotateRadians {
    /// Returns a object implmenting the desired combination of rotations.
    pub fn new(delta_lambda_p: f64, delta_phi: f64, delta_gamma: f64) -> Box<dyn Transform> {
        let delta_lambda = delta_lambda_p % TAU;
        // Should I rotate by lambda, phi or gamma.
        let by_lambda = delta_lambda != 0f64;
        let by_phi = delta_phi != 0f64;
        let by_gamma = delta_gamma != 0f64;
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
