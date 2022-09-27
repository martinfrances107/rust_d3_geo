use geo::CoordFloat;
use geo::Coordinate;

use crate::projection::CenterGet;

use super::Builder;

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
        Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        }
    }
}
