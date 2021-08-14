use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::rejoin::intersection::Intersection;

// Intersections are sorted along the clip edge. For both antimeridian cutting
// and circle clipPIng, the same comparison is used.
// fn compare_intersection(&self, _a: Self::SctCi, _b: Self::SctCi) -> Self::SctT;
pub fn compare_intersections<T>(
    a: &Rc<RefCell<Intersection<T>>>,
    b: &Rc<RefCell<Intersection<T>>>,
) -> Ordering
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
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
