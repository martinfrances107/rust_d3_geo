use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::Builder;
use crate::projection::builder::NoPCNU;
use crate::projection::builder::ResampleNoneNoPCNU;
use crate::projection::builder::ResampleNonePCNU;
use crate::projection::builder::PCNU;
use crate::projection::Recenter;
use crate::projection::ScaleSet;
use crate::Transform;

impl<CLIPU, PR, T> ScaleSet for Builder<CLIPU, NoPCNU, PR, ResampleNoneNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set<CLIPC>(&mut self, scale: T) -> &mut Self {
        self.k = scale;
        self.recenter()
    }
}

impl<CLIPU, PR, T> ScaleSet for Builder<CLIPU, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set<CLIPC>(&mut self, scale: T) -> &mut Self {
        self.k = scale;
        self.recenter()
    }
}
