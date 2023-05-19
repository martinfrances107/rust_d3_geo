use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::template::PCNU;
use crate::projection::ClipExtentAdjust;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPU, PR, RU, T> ClipExtentAdjust for Builder<CLIPU, PCNU<T>, PR, RU, T>
where
    CLIPU: Clone,
    RU: Clone,
    PCNU<T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn clip_extent_adjust<CLIPC>(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.extent = Some(*extent);
        self.reclip::<CLIPC>()
    }
}
