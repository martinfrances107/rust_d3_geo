use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::ScaleSet;
use crate::projection::TransformExtent;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, ResamplePCNU<PR, T>, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    fn scale_set(&mut self, scale: Self::T) -> &mut Self {
        self.base.scale_set(scale);
        self
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, ResampleNonePCNU<PR, T>, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    fn scale_set(&mut self, scale: Self::T) -> &mut Self {
        self.base.scale_set(scale);
        self
    }
}
