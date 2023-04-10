use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::TranslateGet;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T> TranslateGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<T> {
        Coord {
            x: self.x,
            y: self.y,
        }
    }
}
