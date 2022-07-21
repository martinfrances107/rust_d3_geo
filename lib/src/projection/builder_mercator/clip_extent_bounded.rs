use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::PointVisible;
use crate::projection::builder::template::ClipU;
use crate::projection::ClipExtentBounded;
use crate::projection::TransformExtent;

use super::Builder;
use super::Reclip;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentBounded
    for Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
    Self: Reclip,
    PR: TransformExtent<T = T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
    type Output = Self;
    /// f64 or f32.
    type T = T;

    fn clip_extent_clear(mut self) -> Self {
        self.extent = None;
        self.reclip()
    }
}
