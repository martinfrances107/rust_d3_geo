use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::RecenterNoResampling;
use crate::projection::RecenterWithResampling;
use crate::projection::RotateSet;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoPCNC;
use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNoneNoPCNU;
use super::template::ResampleNonePCNC;
use super::template::ResampleNonePCNU;
use super::template::ResamplePCNC;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter_with_resampling()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter_with_resampling()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoneNoPCNC<DRAIN, PR, T>,
        ResampleNoneNoPCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter_no_resampling()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter_no_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResampleNonePCNC<DRAIN, PR, T>,
        ResampleNonePCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter_no_resampling()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter_no_resampling()
    }
}
