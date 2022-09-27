use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::Builder;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleNoneClipU;
use crate::projection::builder::ResampleNoneNoClipU;
use crate::projection::RecenterNoResampling;
use crate::projection::ScaleSet;
use crate::Transform;

use super::template::ResampleNoneClipC;
use super::template::ResampleNoneNoClipC;

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoClipU<DRAIN>,
        PR,
        ResampleNoneNoClipC<DRAIN, PR, T>,
        ResampleNoneNoClipU<DRAIN, PR, T>,
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
        self.recenter_no_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
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

    fn scale_set(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}
