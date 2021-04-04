use geo::CoordFloat;
use num_traits::FloatConst;

use super::rejoin::intersection::Intersection;

pub fn compare_intersection<T>(a: Intersection<T>, b: Intersection<T>) -> T
where
    T: CoordFloat + FloatConst,
{
    let a_dashed = a.x;
    let part1 = match a_dashed.p.x < T::zero() {
        true => a_dashed.p.y - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - a_dashed.p.y,
    };
    let b_dashed = b.x;
    let part2 = match b_dashed.p.x < T::zero() {
        true => b_dashed.p.y - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - b_dashed.p.y,
    };

    return part1 - part2;
}
