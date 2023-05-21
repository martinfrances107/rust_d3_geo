use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::TranslateGet;

use super::Builder;

impl<DRAIN, T> TranslateGet for Builder<DRAIN, T>
where
    T: CoordFloat + Default + FloatConst,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<T> {
        self.pr.lower_48.translate()
    }
}
