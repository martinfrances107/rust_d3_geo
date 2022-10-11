use geo::CoordFloat;
use geo::Coordinate;

use crate::projection::builder_mercator_transverse::Builder;
use crate::projection::TranslateGet;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> TranslateGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coordinate<Self::T> {
        self.base.translate()
    }
}
