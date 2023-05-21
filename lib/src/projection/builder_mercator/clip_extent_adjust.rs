use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::projection::builder::template::PCNU;
use crate::projection::ClipExtentAdjust;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, RU, T> ClipExtentAdjust for Builder<CLIPU, DRAIN, PCNU<T>, PR, RU, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    RU: Clone,
    PCNU<T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.extent = Some(*extent);
        self.reclip()
    }
}
