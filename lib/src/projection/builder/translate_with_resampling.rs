use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::RecenterWithResampling;
use crate::projection::TranslateSet;
use crate::Transform;

use super::template::ClipU;
use super::template::ResampleClipC;
use super::template::ResampleClipU;
use super::Builder;
use super::NoClipU;
use super::ResampleNoClipC;
use super::ResampleNoClipU;

impl<DRAIN, I, LC, LB, LU, PR, PV, T> TranslateSet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        NoClipU<DRAIN>,
        PR,
        PV,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LC, LB, LU, PR, PV, T> TranslateSet
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
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_with_resampling()
    }
}
