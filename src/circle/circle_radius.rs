use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

/// Returns the signed angle of a cartesian point relative to [cosRadius, 0, 0].
pub fn circle_radius<T: CoordFloat + FloatConst>(cos_radius: T, point_p: Coordinate<T>) -> T {
    let mut point = cartesian(&point_p);
    point[0] = point[0] - cos_radius;
    cartesian_normalize_in_place(&mut point);
    let radius = (-point[1]).acos();
    let radius_signed = match -point[2] < T::zero() {
        true => -radius,
        false => radius,
    };
    return radius_signed + T::TAU() - T::epsilon() % T::TAU();
}
