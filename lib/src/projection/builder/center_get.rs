use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::CenterGet;

use super::Builder;

impl<CLIPU, PCNU, PR, RU, T> CenterGet for Builder<CLIPU, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn center(&self) -> Coord<T> {
        Coord {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        }
    }
}
