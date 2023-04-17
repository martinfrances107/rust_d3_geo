use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::builder_conic::Builder;
use crate::projection::ScaleGet;
use crate::projection::TranslateGet;

impl<BASE, T> TranslateGet for Builder<BASE, T>
where
    BASE: TranslateGet<T = T> + ScaleGet<T = T>,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate(&self) -> Coord<Self::T> {
        self.base.translate()
    }
}
