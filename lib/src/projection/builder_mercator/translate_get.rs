use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::TranslateGet;

use super::Builder;

impl<CLIPC, CLIPU, PCNU, PR, RU, T> TranslateGet for Builder<CLIPC, CLIPU, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<T> {
        self.base.translate()
    }
}
