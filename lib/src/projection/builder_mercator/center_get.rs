use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::builder_mercator::Builder;
use crate::projection::CenterGet;

impl<CLIPC, CLIPU, PCNU, PR, RU, T> CenterGet for Builder<CLIPC, CLIPU, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn center(&self) -> Coord<T> {
        self.base.center()
    }
}
