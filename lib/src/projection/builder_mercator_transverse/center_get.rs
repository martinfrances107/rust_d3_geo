use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::builder_mercator_transverse::Builder;
use crate::projection::CenterGet;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> CenterGet for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    // CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn center(&self) -> Coord<T> {
        let c = self.base.center();
        Coord { x: c.y, y: -c.x }
    }
}
