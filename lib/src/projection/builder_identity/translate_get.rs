use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::TranslateGet;

use super::Builder;

impl<DRAIN, PCNU, T> TranslateGet for Builder<DRAIN, PCNU, T>
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
