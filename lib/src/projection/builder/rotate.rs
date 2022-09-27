use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::RecenterWithResampling;
use crate::projection::RotateSet;
use crate::Transform;

use super::template::ClipU;
use super::template::NoClipU;
use super::template::ResampleClipC;
use super::template::ResampleNoClipC;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoClipU<DRAIN>,
        PR,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
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

    /// Sets the rotation angles as measured in degrees.
    fn rotate_set(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        ClipU<DRAIN, T>,
        PR,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
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

    /// Sets the rotation angles as measured in degrees.
    fn rotate_set(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter_with_resampling()
    }
}
