use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::Builder as BuilderCommon;
use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> ClipExtentGet
    for Builder<BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>, PR, T>
where
    BuilderCommon<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>: Clone + ClipExtentGet<T = T>,
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
