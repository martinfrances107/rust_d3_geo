use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::builder_mercator_transverse::Builder;
use crate::projection::TranslateGet;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> TranslateGet for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<Self::T> {
        self.base.translate()
    }
}
