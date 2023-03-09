use geo::Coord;
use geo::CoordFloat;

use super::Builder;

use crate::projection::CenterSet;

impl<BASE, T> CenterSet for Builder<BASE, T>
where
    BASE: CenterSet<T = T>,
    T: CoordFloat,
{
    type T = T;

    fn center_set(&mut self, point: &Coord<Self::T>) -> &mut Self {
        self.base.center_set(point);
        self
    }
}
