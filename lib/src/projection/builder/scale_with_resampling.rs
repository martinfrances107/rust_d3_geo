use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::Builder;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::RecenterWithResampling;
use crate::projection::ScaleSet;
use crate::Transform;

use super::template::ResampleClipC;
use super::template::ResampleNoClipC;

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
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

    fn scale_set(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
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

    fn scale_set(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}
