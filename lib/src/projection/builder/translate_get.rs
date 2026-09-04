use geo::Coord;
use geo::CoordFloat;

use crate::projection::TranslateGet;

use super::Builder;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> TranslateGet
    for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
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
