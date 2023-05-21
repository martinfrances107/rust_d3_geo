use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;

use crate::projection::ScaleGet;
use crate::stream::Stream;

impl<DRAIN, T> ScaleGet for Builder<DRAIN, T>
where
    T: CoordFloat + Default + FloatConst,
{
    type T = T;

    fn scale(&self) -> Self::T {
        self.pr.lower_48.scale()
    }
}
