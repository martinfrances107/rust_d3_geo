use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::clip::rectangle::Rectangle;
use crate::projection::ClipExtentAdjust;
use crate::projection::TransformExtent;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, RU, T> ClipExtentAdjust
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, RU, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    RU: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    fn clip_extent_adjust(&mut self, extent: &[Coord<T>; 2]) -> &mut Self {
        self.extent = Some(*extent);
        self.reclip()
    }
}
