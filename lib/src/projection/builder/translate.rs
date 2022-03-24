use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::Transform;
use crate::projection::Translate;
use crate::stream::Connected;
use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Translate
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
    PV: Clone + Debug,
    PCNC: Clone + Debug,
    PCNU: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PR: Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Translate
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
    PR: Clone,
    LB: Clone,
    LU: Clone + Debug,
    PV: Clone + Debug,
    PCNU: Clone + Debug,
    DRAIN: Clone + Debug,
    LC: Clone + Debug,
    I: Clone,
    PR: Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_no_resampling()
    }
}
