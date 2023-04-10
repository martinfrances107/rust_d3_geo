use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::Builder;
use crate::projection::builder::NoPCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::builder::PCNU;
use crate::projection::Recenter;
use crate::projection::ScaleSet;
use crate::Transform;

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<CLIPC, CLIPU, DRAIN, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.k = scale;
        self.recenter()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ScaleSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.k = scale;
        self.recenter()
    }
}
