use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::Recenter;
use crate::projection::RotateSet;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoneNoPCNU;
use super::template::ResampleNonePCNU;
use super::template::PCNU;
use super::Builder;

impl<CLIPU, DRAIN, PR, T> RotateSet for Builder<CLIPU, DRAIN, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter()
    }
}

impl<CLIPU, DRAIN, PR, T> RotateSet for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter()
    }
}

impl<CLIPU, DRAIN, PR, T> RotateSet
    for Builder<CLIPU, DRAIN, NoPCNU, PR, ResampleNoneNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter()
    }
}

impl<CLIPU, DRAIN, PR, T> RotateSet
    for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        let [delta_lambda, delta_phi] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = T::zero();
        self.recenter()
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        self.delta_lambda = (delta_lambda % self.t360).to_radians();
        self.delta_phi = (delta_phi % self.t360).to_radians();
        self.delta_gamma = (delta_gamma % self.t360).to_radians();
        self.recenter()
    }
}
