use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;
use crate::stream::Unconnected;

use super::Builder;

impl<CLIPU, DRAIN, PR, RU, T> ClipExtentGet
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, RU, T>
where
    CLIPU: Clone,
    PR: TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
    type T = T;

    /// Returns a bounding box.
    #[inline]
    fn clip_extent(&self) -> [Coord<Self::T>; 2] {
        self.base.extent.unwrap()
    }
}
