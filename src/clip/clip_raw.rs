use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::CompareIntersection;
use crate::stream::Stream;

use super::antimeridian::ClipAntimeridian;
use super::circle::ClipCircle;
use super::rejoin::intersection::Intersection;
use super::ClipTraitRaw;

#[derive(Clone, Debug)]
pub enum ClipRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    Antimeridian(ClipAntimeridian<T>),
    Circle(ClipCircle<T>),
}

impl<T> Default for ClipRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fn default() -> Self {
        ClipRaw::Antimeridian(ClipAntimeridian::default())
    }
}

impl<T> ClipTraitRaw<T> for ClipRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    type SctCi = CompareIntersection<T>;

    fn point_visible(&self, p: &Self::SctC, m: Option<u8>) -> bool {
        match self {
            ClipRaw::Antimeridian(c) => c.point_visible(p, m),
            ClipRaw::Circle(c) => c.point_visible(p, m),
        }
    }

    // Intersections are sorted along the clip edge. For both antimeridian cutting
    // and circle clipPIng, the same comparison is used.
    // fn compare_intersection(&self, _a: Self::SctCi, _b: Self::SctCi) -> Self::SctT;
    fn compare_intersection(a: &Intersection<T>, b: &Intersection<T>) -> Ordering {
        let ax = a.x;
        let part1 = match ax.p.x < T::zero() {
            true => ax.p.y - T::FRAC_PI_2() - T::epsilon(),
            false => T::FRAC_PI_2() - ax.p.y,
        };
        let bx = b.x;
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
    ) {
        match self {
            ClipRaw::Antimeridian(c) => c.interpolate(from, to, direction, stream),
            ClipRaw::Circle(c) => c.interpolate(from, to, direction, stream),
        };
    }
}
