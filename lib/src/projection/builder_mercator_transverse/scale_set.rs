use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::clipper::Connectable as ConnectableClip;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::ScaleSet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
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
    for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
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
