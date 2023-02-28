use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::ClipExtentAdjust;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;

impl<BASE, PR, T> ClipExtentAdjust for Builder<BASE, PR, T>
where
    BASE: ClipExtentAdjust<T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.base.clip_extent_adjust(extent);
        self
    }
}
