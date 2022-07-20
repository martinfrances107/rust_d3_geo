use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::ReclipAdjust;
use super::ScaleReclip;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ScaleReclip
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
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = Self;
    type T = T;

    fn scale_reclip(mut self, scale: T) -> Self::Output {
        self.base.k = scale;
        self.reclip_adjust()
    }
}
