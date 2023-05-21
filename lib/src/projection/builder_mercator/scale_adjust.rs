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
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    PCNU<T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.base.scale_set(scale);
        self.reclip()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    CLIPU: Clone + ConnectableClip<Output = CLIPC>,
    PCNU<T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.base.scale_set(scale);
        self.reclip()
    }
}
