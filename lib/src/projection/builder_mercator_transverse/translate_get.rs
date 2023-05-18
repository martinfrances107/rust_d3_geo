use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::builder_mercator_transverse::Builder;
use crate::projection::TranslateGet;

impl<CLIPC, CLIPU, PCNU, PR, RU, T> TranslateGet for Builder<CLIPC, CLIPU, PCNU, PR, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<Self::T> {
        self.base.translate()
    }
}
