use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::TransformExtent;
use crate::stream::Stream;
use crate::Transform;

use super::types::BuilderMercatorAntimeridianResampleNoneClip;
use super::types::BuilderMercatorAntimeridianResampleNoneNoClip;
use super::Reclip;
use super::ScaleReclip;

impl<DRAIN, PR, T> ScaleReclip for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>;
    type T = T;

    fn scale_reclip(mut self, scale: T) -> Self::Output {
        self.base.k = scale;
        self.reclip()
    }
}

impl<DRAIN, PR, T> ScaleReclip for BuilderMercatorAntimeridianResampleNoneNoClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    fn scale_reclip(mut self, scale: T) -> Self::Output {
        self.base.k = scale;
        self.reclip()
    }
}
