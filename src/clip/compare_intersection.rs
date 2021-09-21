use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use super::intersection::Intersection;

/// Intersections are sorted along the clip edge. For both antimeridian cutting
/// and circle clipping, the same comparison is used.
pub fn compare_intersection<T>(
    a: &Rc<RefCell<Intersection<T>>>,
    b: &Rc<RefCell<Intersection<T>>>,
) -> Ordering
where
    T: CoordFloat + FloatConst,
{
    // TODO the JS version uses 1e-6 for epsilon
    // T::epsilon() is way smaller!
    let ax = a.borrow().x;
    let part1 = match ax.p.x < T::zero() {
        true => ax.p.y - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - ax.p.y,
    };
    let bx = b.borrow().x;
    let part2 = match bx.p.x < T::zero() {
        true => bx.p.y - T::FRAC_PI_2() - T::epsilon(),
        false => T::FRAC_PI_2() - bx.p.y,
    };

    let diff = part1 - part2;
    if diff > T::zero() {
        return Ordering::Greater;
    }
    if diff < T::zero() {
        return Ordering::Less;
    }
    Ordering::Equal
}
