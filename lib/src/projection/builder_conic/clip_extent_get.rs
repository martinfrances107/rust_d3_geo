use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::ClipExtentGet;
use crate::projection::ScaleGet;

use super::Builder;

impl<BASE, T> ClipExtentGet for Builder<BASE, T>
where
    BASE: ClipExtentGet<T = T> + ScaleGet<T = T>,
    T: CoordFloat,
{
    /// f32 or f64
    type T = T;

    /// Returns a bounding box.
    #[inline]
    fn clip_extent(&self) -> [Coord<Self::T>; 2] {
        self.base.clip_extent()
    }
}
