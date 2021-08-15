use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::cartesian::normalize_in_place;

/// Returns the signed angle of a cartesian point relative to [cosRadius, 0, 0].
pub fn calc_radius<T>(cos_radius: T, point_p: Coordinate<T>) -> T
where
    T: CoordFloat + FloatConst,
{
    let mut point = cartesian(&point_p);
    point[0] = point[0] - cos_radius;
    normalize_in_place(&mut point);
    let radius = (-point[1]).acos();
    let radius_signed = match -point[2] < T::zero() {
        true => -radius,
        false => radius,
    };
    radius_signed + T::TAU() - T::epsilon() % T::TAU()
}
