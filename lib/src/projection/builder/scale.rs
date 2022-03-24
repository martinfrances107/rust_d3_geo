use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::Transform;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::Scale;
use crate::stream::Connected;
use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Scale
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
        Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    DRAIN: Clone + Debug,
    I: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PV: Clone + Debug,
    PCNC: Clone + Debug,
    PCNU: Clone + Debug,
    PR: Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_scale(&self) -> Self::T {
        self.k
    }

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Scale
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
    I: Clone,
    DRAIN: Clone + Debug,
    PCNU: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PV: Clone + Debug,
    PR: Clone + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_scale(&self) -> Self::T {
        self.k
    }

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}
