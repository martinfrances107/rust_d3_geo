use approx::AbsDiffEq;

use geo::CoordFloat;
use num_traits::FloatConst;
use std::fmt::Debug;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::RotateSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
use super::template::ClipC;
use super::template::ClipU;
use super::template::NoClipC;
use super::template::NoClipU;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> RotateSet
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
    DRAIN: Debug,
    LB: Debug,
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    fn rotate(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU,  PR, PV, T> RotateSet
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
        Resample<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, ConnectedResample<ClipC<DRAIN, T>, T>, T>,
        Resample<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Unconnected, T>,
        T,
    >
where
    DRAIN: Debug,
    LB: Debug,
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    fn rotate(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter_with_resampling()
    }
}

// TODO must vary by Clip/NoClip
impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> RotateSet
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
    DRAIN: Debug,
    PCNC: Debug,
    PCNU: Debug,
    PR: Clone + Debug + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    fn rotate(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter_no_resampling()
    }
}
