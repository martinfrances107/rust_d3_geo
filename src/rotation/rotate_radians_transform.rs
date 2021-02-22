use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::Transform;

use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

pub fn rotate_radians_transform<T: CoordFloat + FloatConst + std::default::Default + 'static>(
    delta_lambda_p: T,
    delta_phi: T,
    delta_gamma: T,
) -> Box<dyn Transform<C = Coordinate<T>>> {
    let delta_lambda = delta_lambda_p % T::TAU();
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
        (false, false, false) => Box::new(RotationIdentity::default()),
    };
}
