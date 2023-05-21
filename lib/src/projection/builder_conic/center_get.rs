use geo::Coord;
use geo::CoordFloat;

use super::Builder;

use crate::projection::CenterGet;
use crate::projection::ScaleGet;

impl<BASE, T> CenterGet for Builder<BASE, T>
where
    BASE: CenterGet<T = T> + ScaleGet<T = T>,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn center(&self) -> Coord<Self::T>
    where
        Self::T: CoordFloat,
    {
        self.base.center()
    }
}
