use geo::Coord;
use geo::CoordFloat;

use crate::projection::CenterSet;
use crate::projection::ScaleGet;

use super::Builder;

impl<BASE, T> CenterSet for Builder<BASE, T>
where
    BASE: CenterSet<T = T> + ScaleGet<T = T>,
    T: CoordFloat,
{
    type T = T;

    fn center_set<CLIPC>(&mut self, point: &Coord<Self::T>) -> &mut Self {
        self.base.center_set::<CLIPC>(point);
        self
    }
}
