use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::RecenterWithResampling;
use crate::projection::TranslateSet;
use crate::Transform;

use super::template::ClipU;
use super::template::ResampleClipC;
use super::template::ResampleClipU;
use super::template::ResampleNoClipC;
use super::Builder;
use super::NoClipU;
use super::ResampleNoClipU;

impl<CLIPC, CLIPU, DRAIN, PR, T> TranslateSet
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
    CLIPU: Clone,
    CLIPC: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_with_resampling()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> TranslateSet
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

    fn translate_set(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_with_resampling()
    }
}
