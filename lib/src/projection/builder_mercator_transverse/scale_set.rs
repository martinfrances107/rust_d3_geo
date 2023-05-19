use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::ScaleSet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;

impl<CLIPU, PR, T> ScaleSet for Builder<CLIPU, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    CLIPU: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    fn scale_set<CLIPC>(&mut self, scale: Self::T) -> &mut Self {
        self.base.scale_set::<CLIPC>(scale);
        self
    }
}

impl<CLIPU, PR, T> ScaleSet for Builder<CLIPU, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    CLIPU: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    fn scale_set<CLIPC>(&mut self, scale: Self::T) -> &mut Self {
        self.base.scale_set::<CLIPC>(scale);
        self
    }
}
