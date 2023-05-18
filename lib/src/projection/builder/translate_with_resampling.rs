use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::Recenter;
use crate::projection::TranslateSet;
use crate::Transform;

use super::template::ResamplePCNU;
use super::template::PCNU;
use super::Builder;
use super::NoPCNU;
use super::ResampleNoPCNU;

impl<CLIPC, CLIPU, PR, T> TranslateSet
    for Builder<CLIPC, CLIPU, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter()
    }
}

impl<CLIPC, CLIPU, PR, T> TranslateSet
    for Builder<CLIPC, CLIPU, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter()
    }
}
