use geo::Coord;
use geo::CoordFloat;

use crate::projection::TranslateGet;

use super::Builder;

impl<PCNU, T> TranslateGet for Builder<PCNU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<T> {
        Coord {
            x: self.tx,
            y: self.ty,
        }
    }
}
