use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::ScaleAdjust;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::template::ResampleClipC;
use super::template::ResampleClipU;
use super::Builder;
use super::ClipC;
use super::ClipU;
use super::NoClipC;
use super::NoClipU;
use super::ResampleNoClipC;
use super::ResampleNoClipU;

impl<DRAIN, I, LC, LB, LU, PR, PV, T> ScaleAdjust
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        NoClipC<DRAIN, T>,
        NoClipU<DRAIN, T>,
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

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LC, LB, LU, PR, PV, T> ScaleAdjust
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipC<DRAIN, T>,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> ScaleAdjust
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}
