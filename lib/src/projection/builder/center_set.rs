use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::CenterSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
use super::template::ClipC;
use super::template::ClipU;
use super::template::NoClipC;
use super::template::NoClipU;

impl<DRAIN, I, LB, LC, LU,  PR, PV, T> CenterSet
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
        Resample<DRAIN, PR, ClipC<DRAIN, T>,  ClipU<DRAIN, T>, ConnectedResample<ClipC<DRAIN, T>, T>, T>,
        Resample<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Unconnected, T>,
        T,
    >
where
    PR: Clone + Debug + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU,  PR, PV, T> CenterSet
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
        Resample<DRAIN, PR, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, ConnectedResample<NoClipC<DRAIN, T>, T>, T>,
        Resample<DRAIN, PR, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, Unconnected, T>,
        T,
    >
where
    PR: Clone + Debug + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

// TODO must vary by Clip/NoClip.
impl<DRAIN, INTERPOLATE, LB, LC, LU, PCNC, PCNU, PR, PV, T> CenterSet
    for Builder<
        DRAIN,
        INTERPOLATE,
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
    DRAIN: Debug,
    PCNC: Debug,
    PCNU: Debug,
    PR: Clone + Debug + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_no_resampling()
    }
}
