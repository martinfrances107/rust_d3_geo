use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoneClip;
use crate::projection::TransformExtent;
use crate::projection::Translate;
use crate::stream::Stream;
use crate::Transform;

use super::Reclip;

impl<DRAIN, PR, T> Translate for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.base = self.base.translate(t);
        self.reclip()
    }
}

impl<DRAIN, PR, T> Translate for BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
    PR: Clone + Debug + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.base = self.base.translate(t);
        self.reclip()
    }
}
