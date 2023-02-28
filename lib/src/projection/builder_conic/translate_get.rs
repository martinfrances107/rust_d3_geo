use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::builder_conic::Builder;
use crate::projection::TranslateGet;

impl<BASE, PR, T> TranslateGet for Builder<BASE, PR, T>
where
    BASE: TranslateGet<T = T>,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<Self::T> {
        self.base.translate()
    }
}
