use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::AngleSet;
use crate::projection::RecenterWithResampling;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoPCNC;
use super::template::ResamplePCNC;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU<DRAIN>,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<DRAIN, PR, T>,
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

    fn angle_set(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<DRAIN, T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<DRAIN, PR, T>,
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

    fn angle_set(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}
