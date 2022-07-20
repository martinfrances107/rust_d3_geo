use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::CenterSet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> CenterSet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Clone,
    I: Clone,
    LC: Clone,
    LU: Clone,
    PV: Clone,

    PR: Clone + Debug + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + Debug + FloatConst,
{
    type T = T;

    fn center(mut self, center: &Coordinate<T>) -> Self {
        self.base = self.base.center(center);
        self.reclip()
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> CenterSet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleNoneClipC<DRAIN, PR, T>,
        ResampleNoneClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Clone,
    I: Clone,
    LC: Clone,
    LU: Clone,
    PV: Clone,
    PR: Clone + Debug + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + Debug + FloatConst,
{
    type T = T;

    fn center(mut self, center: &Coordinate<T>) -> Self {
        self.base = self.base.center(center);
        self.reclip()
    }
}
