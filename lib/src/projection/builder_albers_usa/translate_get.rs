use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::TranslateGet;
use crate::stream::Stream;

use super::Builder;

impl<DRAIN, T> TranslateGet for Builder<DRAIN, T>
where
    T: CoordFloat + Default + FloatConst,
    DRAIN: Clone + Stream<EP = DRAIN, T = T>,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<T> {
        self.pr.lower_48.translate()
    }
}
