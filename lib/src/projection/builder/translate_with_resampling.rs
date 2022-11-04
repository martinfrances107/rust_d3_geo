use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::RecenterWithResampling;
use crate::projection::TranslateSet;
use crate::Transform;

use super::template::ResampleNoPCNC;
use super::template::ResamplePCNC;
use super::template::ResamplePCNU;
use super::template::PCNU;
use super::Builder;
use super::NoPCNU;
use super::ResampleNoPCNU;

impl<CLIPC, CLIPU, DRAIN, PR, T> TranslateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<PR, T>,
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

    fn translate_set(&mut self, t: &Coordinate<T>) -> &mut Self {
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
        PCNU<T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<PR, T>,
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
        self.recenter_with_resampling()
    }
}
