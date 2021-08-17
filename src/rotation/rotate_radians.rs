use geo::CoordFloat;

use num_traits::FloatConst;

use crate::compose::Compose;

use super::rotate_radians_enum::RotateRadiansEnum;
use super::rotation_identity::RotationIdentity;
use super::rotation_lambda::RotationLambda;
use super::rotation_phi_gamma::RotationPhiGamma;

pub fn rotate_radians<T>(delta_lambda_p: T, delta_phi: T, delta_gamma: T) -> RotateRadiansEnum<T>
where
    T: CoordFloat + FloatConst,
{
    let delta_lambda = delta_lambda_p % T::TAU();
    // Should I rotate by lambda, phi or gamma.
    let by_lambda = !delta_lambda.is_zero();
    let by_phi = !delta_phi.is_zero();
    let by_gamma = !delta_gamma.is_zero();
    match (by_lambda, by_gamma, by_phi) {
        (true, true, true) | (true, true, false) | (true, false, true) => {
            RotateRadiansEnum::C(Box::new(Compose::new(
                RotationLambda::new(delta_lambda),
                RotationPhiGamma::new(&delta_phi, &delta_gamma),
            )))
        }
        (true, false, false) => RotateRadiansEnum::RL(RotationLambda::new(delta_lambda)),
        (false, true, true) | (false, true, false) | (false, false, true) => {
            RotateRadiansEnum::RPG(RotationPhiGamma::new(&delta_phi, &delta_gamma))
        }
        (false, false, false) => RotateRadiansEnum::I(RotationIdentity::default()),
    }
}
