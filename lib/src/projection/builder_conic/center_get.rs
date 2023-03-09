use geo::Coord;
use geo::CoordFloat;

use super::Builder;

use crate::projection::CenterGet;

impl<BASE, T> CenterGet for Builder<BASE, T>
where
    BASE: CenterGet<T = T>,
    T: CoordFloat,
{
    type T = T;

    fn center(&self) -> Coord<Self::T>
    where
        Self::T: CoordFloat,
    {
        self.base.center()
    }
}
