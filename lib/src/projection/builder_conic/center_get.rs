use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;

use crate::projection::CenterGet;

impl<BASE, T> CenterGet for Builder<BASE, T>
where
    BASE: CenterGet<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center(&self) -> Coord<Self::T>
    where
        Self::T: CoordFloat,
    {
        self.base.center()
    }
}
