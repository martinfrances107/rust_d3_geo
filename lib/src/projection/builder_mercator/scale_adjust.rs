use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::ScaleSet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<DRAIN, T>,
        PR,
        ResampleNonePCNC<DRAIN, PR, T>,
        ResampleNonePCNU<DRAIN, PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PCNU<DRAIN, T>: Clone,
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
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<DRAIN, T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<DRAIN, PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PCNU<DRAIN, T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.base.scale_set(scale);
        self.reclip()
    }
}
