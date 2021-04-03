use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::stream::CompareIntersection;
use crate::stream::Stream;

use super::antimeridian::ClipAntimeridian;
use super::circle::ClipCircle;
use super::ClipTraitRaw;

#[derive(Clone, Debug)]
pub enum ClipRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    Antimeridian(ClipAntimeridian<T>),
    Circle(ClipCircle<T>),
}

impl<T> Default for ClipRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        ClipRaw::Antimeridian(ClipAntimeridian::default())
    }
}

impl<T> ClipTraitRaw<T> for ClipRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
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
    fn compare_intersection(&self, a: Self::SctCi, b: Self::SctCi) -> Self::SctT {
        match self {
            ClipRaw::Antimeridian(c) => c.compare_intersection(a, b),
            ClipRaw::Circle(c) => c.compare_intersection(a, b),
        }
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
