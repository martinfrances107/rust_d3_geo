use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::AngleSet;
use crate::projection::RecenterWithResampling;
use crate::Transform;

use super::template::ClipU;
use super::template::NoClipU;
use super::template::ResampleClipC;
use super::template::ResampleNoClipC;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
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

    fn angle_set(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}
