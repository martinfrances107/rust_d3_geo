use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::ClipExtentAdjust;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::ReclipAdjust;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentAdjust
    for Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
    I: Clone,
    LC: Clone,
    LU: Clone,
    PV: Clone,
    RC: Clone,
    RU: Clone,
    ClipU<DRAIN, T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn clip_extent_adjust(mut self, extent: &[Coordinate<T>; 2]) -> Self {
        self.extent = Some(*extent);
        self.reclip_adjust()
    }
}
