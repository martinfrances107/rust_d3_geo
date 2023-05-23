use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rejoin::CompareIntersectionsFn;
use crate::math::EPSILON;

use super::intersection::Intersection;

/// Intersections are sorted along the clip edge. For both antimeridian cutting
/// and circle clipping, the same comparison is used.
///
/// # Panics
/// unwrap() is used here but a panic will never happen as EPSILON will always be converted into T.
#[must_use]
pub(super) fn gen_compare<T>() -> CompareIntersectionsFn<T>
where
    T: 'static + CoordFloat + FloatConst,
{
    let epsilon = T::from(EPSILON).unwrap();
    Box::new(
        move |a: &Rc<RefCell<Intersection<T>>>, b: &Rc<RefCell<Intersection<T>>>| -> Ordering {
            let ax = a.borrow().x;
            let part1 = if ax.p.x < T::zero() {
                ax.p.y - T::FRAC_PI_2() - epsilon
            } else {
                T::FRAC_PI_2() - ax.p.y
            };
            let bx = b.borrow().x;

            let part2 = if bx.p.x < T::zero() {
                bx.p.y - T::FRAC_PI_2() - epsilon
            } else {
                T::FRAC_PI_2() - bx.p.y
            };

            let diff = part1 - part2;
            if diff > T::zero() {
                Ordering::Greater
            } else if diff < T::zero() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        },
    )
}
