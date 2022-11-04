use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::ResampleNoPCNC;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::ResampleNoPCNU;
use crate::projection::AngleGet;
use crate::projection::AngleSet;
use crate::Transform;

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> AngleGet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn angle(&self) -> T {
        self.base.angle()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    #[inline]
    fn angle_set(&mut self, angle: T) -> &mut Self {
        self.base.angle_set(angle);
        self
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the rotation angles as measured in degrees.
    #[inline]
    fn angle_set(&mut self, angle: T) -> &mut Self {
        self.base.angle_set(angle);
        self
    }
}
