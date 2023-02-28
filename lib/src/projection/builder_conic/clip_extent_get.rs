use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;

use super::Builder;

impl<BASE, PR, T> ClipExtentGet for Builder<BASE, PR, T>
where
    BASE: ClipExtentGet<T = T>,
    PR: TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    /// Returns a bounding box.
    #[inline]
    fn clip_extent(&self) -> [Coord<Self::T>; 2] {
        self.base.clip_extent()
    }
}
