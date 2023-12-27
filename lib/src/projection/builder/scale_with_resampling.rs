use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::Builder;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::Recenter;
use crate::projection::ScaleSet;
use crate::stream::Unconnected;
use crate::Transform;

impl<CLIPU, DRAIN, PR, T> ScaleSet
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

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.k = scale;
        self.recenter()
    }
}

impl<CLIPU, DRAIN, PR, T> ScaleSet
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

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.k = scale;
        self.recenter()
    }
}
