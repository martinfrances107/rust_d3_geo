use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::AngleSet;
use crate::projection::Recenter;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

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

    fn angle_set(&mut self, angle: T) -> &mut Self {
        self.alpha = (angle % self.t360).to_radians();
        self.recenter()
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

    fn angle_set(&mut self, angle: T) -> &mut Self {
        self.alpha = (angle % self.t360).to_radians();
        self.recenter()
    }
}
