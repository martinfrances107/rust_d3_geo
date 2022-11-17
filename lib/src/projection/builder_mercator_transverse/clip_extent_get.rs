use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::template::PCNU;
use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, RC, RU, T>
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
    fn clip_extent(&self) -> Option<[Coord<Self::T>; 2]> {
        self.base.extent
    }
}
