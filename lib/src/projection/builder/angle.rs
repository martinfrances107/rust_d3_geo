use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::AngleSet;
use crate::projection::Recenter;
use crate::Transform;

use super::template::NoPCNU;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
    for Builder<CLIPC, CLIPU, DRAIN, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn angle_set(&mut self, angle: T) -> &mut Self {
        self.alpha = (angle % self.t360).to_radians();
        self.recenter()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn angle_set(&mut self, angle: T) -> &mut Self {
        self.alpha = (angle % self.t360).to_radians();
        self.recenter()
    }
}
