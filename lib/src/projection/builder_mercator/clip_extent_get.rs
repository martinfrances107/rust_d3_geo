use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::template::PCNU;
use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;

use super::Builder;

impl<CLIPU, PR, RU, T> ClipExtentGet for Builder<CLIPU, PCNU<T>, PR, RU, T>
where
    CLIPU: Clone,
    PR: TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Returns a bounding box.
    #[inline]
    fn clip_extent(&self) -> [Coord<Self::T>; 2] {
        self.extent.unwrap()
    }
}
