use geo::CoordFloat;
use geo::Coordinate;

use crate::projection::builder_mercator::Builder;
use crate::projection::CenterGet;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> CenterGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn center(&self) -> Coordinate<T> {
        self.base.center()
    }
}
