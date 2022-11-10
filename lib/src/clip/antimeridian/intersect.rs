use geo::CoordFloat;

use crate::math::EPSILON;

pub fn intersect<T: CoordFloat>(lambda0: T, phi0: T, lambda1: T, phi1: T) -> T {
    let sin_lambda0_lambda1 = (lambda0 - lambda1).sin();
    if (sin_lambda0_lambda1).abs() > T::from(EPSILON).unwrap() {
        let (sin_phi0, cos_phi0) = phi0.sin_cos();
        let (sin_phi1, cos_phi1) = phi1.sin_cos();
        ((sin_phi0 * cos_phi1 * lambda1.sin() - sin_phi1 * cos_phi0 * lambda0.sin())
            / (cos_phi0 * cos_phi1 * sin_lambda0_lambda1))
            .tan()
    } else {
        (phi0 + phi1) / T::from(2).unwrap()
    }
}
