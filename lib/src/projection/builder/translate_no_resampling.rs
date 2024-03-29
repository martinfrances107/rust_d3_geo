use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::ResampleNoneNoPCNU;
use crate::projection::builder::ResampleNonePCNU;
use crate::projection::Recenter;
use crate::projection::TranslateSet;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPU, DRAIN, PR, T> TranslateSet
    for Builder<
        CLIPU,
        DRAIN,
        Identity<Unconnected>,
        PR,
        ResampleNoneNoPCNU<PR, T>,
        T,
    >
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

impl<CLIPU, DRAIN, PR, T> TranslateSet
    for Builder<
        CLIPU,
        DRAIN,
        Rectangle<Unconnected, T>,
        PR,
        ResampleNonePCNU<PR, T>,
        T,
    >
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
