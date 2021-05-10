pub mod antimeridian;
pub mod circle;
pub mod clip;
pub mod clip_base;
pub mod clip_buffer;
pub mod clip_raw;
pub mod clip_sink_enum;
pub mod line_elem;
pub mod line_enum;
pub mod line_sink_enum;
pub mod rejoin;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;

use clip_buffer::ClipBuffer;
use rejoin::intersection::Intersection;

pub trait ClipTraitRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SctC;
    type SctOC;
    type SctT: CoordFloat + FloatConst;
    type SctCi;

    fn point_visible(&self, _p: &Self::SctC, _z: Option<u8>) -> bool;

    /// Intersections are sorted along the clip edge. For both antimeridian cutting
    /// and circle clipPing, the same comparison is used.    
    fn compare_intersection(
        a: &Rc<RefCell<Intersection<T>>>,
        b: &Rc<RefCell<Intersection<T>>>,
    ) -> Ordering {
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

    fn interpolate(
        &self,
        from: Self::SctOC,
        to: Self::SctOC,
        direction: Self::SctT,
        stream: &mut impl Stream<T, C = Coordinate<T>>,
    );
}
