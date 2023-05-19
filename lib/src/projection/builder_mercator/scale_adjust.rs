use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::ScaleSet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPU, PR, T> ScaleSet for Builder<CLIPU, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    CLIPU: Clone,
    PCNU<T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set<CLIPC>(&mut self, scale: T) -> &mut Self {
        self.base.scale_set::<CLIPC>(scale);
        self.reclip::<CLIPC>()
    }
}

impl<CLIPU, PR, T> ScaleSet for Builder<CLIPU, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    CLIPU: Clone,
    PCNU<T>: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set<CLIPC>(&mut self, scale: T) -> &mut Self {
        self.base.scale_set::<CLIPC>(scale);
        self.reclip::<CLIPC>()
    }
}
