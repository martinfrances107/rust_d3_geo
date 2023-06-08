use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder_mercator::Reclip;
use crate::projection::TransformExtent;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, RU, T> Reclip
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, RU, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    RU: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    fn reclip(&mut self) -> &mut Self {
        self.base.reclip();
        self
    }
}
