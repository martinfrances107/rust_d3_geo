use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::CenterSet;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
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

    PR: Clone + Debug + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(mut self, center: &Coordinate<T>) -> Self {
        self.base = self.base.center_set(center);
        self.reclip()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> CenterSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        ClipU<DRAIN, T>,
        PR,
        ResampleNoneClipC<DRAIN, PR, T>,
        ResampleNoneClipU<DRAIN, PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Debug + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn center_set(mut self, center: &Coordinate<T>) -> Self {
        self.base = self.base.center_set(center);
        self.reclip()
    }
}
