use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::CenterSet;
use crate::projection::RecenterNoResampling;
use crate::projection::RecenterWithResampling;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoPCNC;
use super::template::ResampleNonePCNC;
use super::template::ResampleNonePCNU;
use super::template::ResamplePCNC;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
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

    fn center_set(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
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

    fn center_set(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<DRAIN, T>,
        PR,
        ResampleNonePCNC<DRAIN, PR, T>,
        ResampleNonePCNU<DRAIN, PR, T>,
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

    fn center_set(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_no_resampling()
    }
}
