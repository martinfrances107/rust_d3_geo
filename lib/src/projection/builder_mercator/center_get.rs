use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder_mercator::Builder;
use crate::projection::CenterGet;

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> CenterGet
    for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
    PCNU: Debug,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        self.base.get_center()
    }
}
