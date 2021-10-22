use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use super::intersection::Intersection;
use crate::clip::rejoin::CompareIntersectionsFn;
use crate::math::EPSILON;

/// Intersections are sorted along the clip edge. For both antimeridian cutting
/// and circle clipping, the same comparison is used.
pub fn gen_compare_intersection<T>() -> CompareIntersectionsFn<T>
where
    T: CoordFloat + FloatConst,
{
    Box::new(
        |a: &Rc<RefCell<Intersection<T>>>, b: &Rc<RefCell<Intersection<T>>>| -> Ordering {
            let epsilon = T::from(EPSILON).unwrap();
            let ax = a.borrow().x;
            let part1 = match ax.p.x < T::zero() {
                true => ax.p.y - T::FRAC_PI_2() - epsilon,
                false => T::FRAC_PI_2() - ax.p.y,
            };
            let bx = b.borrow().x;
            let part2 = match bx.p.x < T::zero() {
                true => bx.p.y - T::FRAC_PI_2() - epsilon,
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
        },
    )
}
