use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::CenterSet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,

    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(&mut self, center: &Coord<T>) -> &mut Self {
        self.base.center_set(&Coord {
            x: -center.y,
            y: center.x,
        });
        self
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(&mut self, center: &Coord<T>) -> &mut Self {
        self.base.center_set(&Coord {
            x: -center.y,
            y: center.x,
        });
        self
    }
}
