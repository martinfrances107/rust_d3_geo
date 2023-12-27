use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::ResampleNoPCNU;
use crate::projection::AngleGet;
use crate::projection::AngleSet;
use crate::stream::Unconnected;
use crate::Transform;

impl<CLIPU, DRAIN, PCNU, PR, RU, T> AngleGet
    for Builder<CLIPU, DRAIN, PCNU, PR, RU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn angle(&self) -> T {
        self.base.angle()
    }
}

impl<CLIPU, DRAIN, PR, T> AngleSet
    for Builder<
        CLIPU,
        DRAIN,
        Identity<Unconnected>,
        PR,
        ResampleNoPCNU<PR, T>,
        T,
    >
where
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

impl<CLIPU, DRAIN, PR, T> AngleSet
    for Builder<
        CLIPU,
        DRAIN,
        Rectangle<Unconnected, T>,
        PR,
        ResamplePCNU<PR, T>,
        T,
    >
where
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
