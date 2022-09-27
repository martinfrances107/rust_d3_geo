use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::ClipExtentAdjust;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentAdjust
    for Builder<CLIPC, CLIPU, DRAIN, ClipU<DRAIN, T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    RU: Clone,
    ClipU<DRAIN, T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn clip_extent_adjust(mut self, extent: &[Coordinate<T>; 2]) -> Self {
        self.extent = Some(*extent);
        self.reclip()
    }
}
