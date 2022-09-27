use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::ResampleNoneClipU;
use crate::projection::builder::ResampleNoneNoClipU;
use crate::projection::RecenterNoResampling;
use crate::projection::TranslateSet;
use crate::Transform;

use super::template::ClipU;
use super::template::ResampleNoneClipC;
use super::template::ResampleNoneNoClipC;
use super::Builder;
use super::NoClipU;

impl<CLIPC, CLIPU, DRAIN, PR, T> TranslateSet
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

    fn translate_set(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_no_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> TranslateSet
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

    fn translate_set(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_no_resampling()
    }
}
