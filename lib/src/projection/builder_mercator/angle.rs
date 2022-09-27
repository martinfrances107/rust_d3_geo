use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::ResampleNoClipU;
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
        NoClipU<DRAIN>,
        PR,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
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
    fn angle_set(self, angle: T) -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
            p_clipc: PhantomData::<CLIPC>,
            p_rc: PhantomData::<ResampleNoClipC<DRAIN, PR, T>>,
            extent: self.extent, // post-clip extent
            pr: self.pr,
            base: self.base.angle_set(angle),
        }
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> AngleSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        ClipU<DRAIN, T>,
        PR,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
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
    fn angle_set(self, angle: T) -> Self {
        Self {
            p_clipc: PhantomData::<CLIPC>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<ResampleClipC<DRAIN, PR, T>>,
            extent: self.extent,
            pr: self.pr,
            base: self.base.angle_set(angle),
        }
    }
}
