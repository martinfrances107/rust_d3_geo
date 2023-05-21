use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::builder_mercator::Builder;
use crate::projection::CenterGet;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> CenterGet for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn center(&self) -> Coord<T> {
        self.base.center()
    }
}
