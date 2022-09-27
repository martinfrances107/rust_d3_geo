use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::CenterSet;
use crate::projection::RecenterNoResampling;
use crate::projection::RecenterWithResampling;
use crate::Transform;

use super::template::ClipU;
use super::template::NoClipU;
use super::template::ResampleClipC;
use super::template::ResampleNoClipC;
use super::template::ResampleNoneClipC;
use super::template::ResampleNoneClipU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
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
        ClipU<DRAIN, T>,
        PR,
        ResampleNoneClipC<DRAIN, PR, T>,
        ResampleNoneClipU<DRAIN, PR, T>,
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
