use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::builder_mercator::Builder;
use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentGet
    for Builder<CLIPC, CLIPU, DRAIN, ClipU<DRAIN, T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    PR: TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    /// Returns a bounding box.
    #[inline]
    fn clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]> {
        self.extent
    }
}
