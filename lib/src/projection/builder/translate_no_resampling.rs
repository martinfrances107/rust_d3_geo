use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::ResampleNoneNoPCNU;
use crate::projection::builder::ResampleNonePCNU;
use crate::projection::RecenterNoResampling;
use crate::projection::TranslateSet;
use crate::Transform;

use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNonePCNC;
use super::template::PCNU;
use super::Builder;
use super::NoPCNU;

impl<CLIPC, CLIPU, DRAIN, PR, T> TranslateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU<DRAIN>,
        PR,
        ResampleNoneNoPCNC<DRAIN, PR, T>,
        ResampleNoneNoPCNU<DRAIN, PR, T>,
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

    fn translate_set(&mut self, t: &Coordinate<T>) -> &mut Self {
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

    fn translate_set(&mut self, t: &Coordinate<T>) -> &mut Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter_no_resampling()
    }
}
