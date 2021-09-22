/// Output type for a 3-axis rotation.
pub mod rotate_radians;
/// The rotation transform (in degrees).
pub mod rotation;
/// An Inner type for the 3-axis rotation.
pub mod rotation_identity;
/// An Inner type for the 3-axis rotation.
pub mod rotation_lambda;
/// An Inner type for the 3-axis rotation.
pub mod rotation_phi_gamma;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::compose::Compose;

use rotate_radians::RotateRadians;
use rotation_identity::RotationIdentity;
use rotation_lambda::RotationLambda;
use rotation_phi_gamma::RotationPhiGamma;

/// Construct a 3-axis rotation transform.
pub fn rotate_radians<T>(delta: [T; 3]) -> RotateRadians<T>
where
    T: CoordFloat + FloatConst,
{
    let delta_lambda = delta[0] % T::TAU();
    let delta_phi = delta[1];
    let delta_gamma = delta[2];
    // Should I rotate by lambda, phi or gamma.
    let by_lambda = !delta_lambda.is_zero();
    let by_phi = !delta[1].is_zero();
    let by_gamma = !delta[2].is_zero();
    match (by_lambda, by_gamma, by_phi) {
        (true, true, true) | (true, true, false) | (true, false, true) => {
            RotateRadians::C(Box::new(Compose::new(
                RotationLambda::new(delta_lambda),
                RotationPhiGamma::new(&delta_phi, &delta_gamma),
            )))
        }
        (true, false, false) => RotateRadians::RL(RotationLambda::new(delta_lambda)),
        (false, true, true) | (false, true, false) | (false, false, true) => {
            RotateRadians::RPG(RotationPhiGamma::new(&delta_phi, &delta_gamma))
        }
        (false, false, false) => RotateRadians::I(RotationIdentity::default()),
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::in_delta::in_delta;
    use crate::Transform;
    use geo::Coordinate;
    use rotation::Rotation;

    #[test]
    fn only_longitude() {
        println!("a rotation of [+90°, 0°] only rotates longitude");
        let rotation = Rotation::new(90f64, 0f64, 0f64).transform(&Coordinate { x: 0f64, y: 0f64 });
        assert!(in_delta(rotation.x, 90f64, 1e-6));
        assert!(in_delta(rotation.y, 0f64, 1e-6));
    }

    #[test]
    fn wraps_antimeridan() {
        println!("a rotation of [+90°, 0°] wraps around when crossing the antimeridian");
        let rotation =
            Rotation::new(90f64, 0f64, 0f64).transform(&Coordinate { x: 150f64, y: 0f64 });
        assert!(in_delta(rotation.x, -120_f64, 1e-6));
        assert!(in_delta(rotation.y, 0_f64, 1e-6));
    }

    #[test]
    fn rotation_long_and_lat() {
        println!("a rotation of [-45°, 45°] rotates longitude and latitude");
        let rotation =
            Rotation::new(-45f64, 45f64, 0f64).transform(&Coordinate { x: 0f64, y: 0f64 });
        assert!(in_delta(rotation.x, -54.73561_f64, 1e-6));
        assert!(in_delta(rotation.y, 30_f64, 1e-6));
    }

    #[test]
    fn rotation_inverse_long_lat() {
        println!("a rotation of [-45°, 45°] rotates longitude and latitude");
        let rotation = Rotation::new(-45_f64, 45_f64, 0_f64).invert(&Coordinate {
            x: -54.73561_f64,
            y: 30_f64,
        });
        assert!(in_delta(rotation.x, 0f64, 1e-6));
        assert!(in_delta(rotation.y, 0f64, 1e-6));
    }

    #[test]
    fn identity_rotation() {
        println!("the identity rotation constrains longitudes to [-180°, 180°]");
        let rotate = Rotation::new(0f64, 0f64, 0f64);
        assert_eq!(
            rotate.transform(&Coordinate { x: 180f64, y: 0f64 }).x,
            180f64
        );
        assert_eq!(
            rotate
                .transform(&Coordinate {
                    x: -180f64,
                    y: 0f64
                })
                .x,
            -180f64
        );
        assert_eq!(rotate.transform(&Coordinate { x: 360f64, y: 0f64 }).x, 0f64);
        assert!(in_delta(
            rotate
                .transform(&Coordinate {
                    x: 2562f64,
                    y: 0f64
                })
                .x,
            42f64,
            1e-10
        ));
        assert!(in_delta(
            rotate
                .transform(&Coordinate {
                    x: -2562f64,
                    y: 0f64
                })
                .x,
            -42f64,
            1e-10
        ));
    }
}
