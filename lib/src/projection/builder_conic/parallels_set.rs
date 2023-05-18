use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use super::PRConic;
use super::ParallelsSet;
use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::ResampleNoPCNU;
use crate::projection::builder::Builder as BuilderCommon;
use crate::projection::Recenter;

// Reach into builder and alter the PR.
impl<CLIPC, CLIPU, PR, T> ParallelsSet
    for Builder<BuilderCommon<CLIPC, CLIPU, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>, T>
where
    PR: PRConic<T = T> + Clone,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn parallels_set(&mut self, phi0: T, phi1: T) -> &mut Self {
        self.phi0 = phi0.to_radians();
        self.phi1 = phi1.to_radians();
        self.base.projection_raw = self
            .base
            .projection_raw
            .clone()
            .generate(self.phi0, self.phi1);
        self.base.recenter();
        self
    }
}
