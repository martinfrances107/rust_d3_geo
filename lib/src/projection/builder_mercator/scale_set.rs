use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorCircleResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorCircleResampleNoClip;
use crate::projection::builder_mercator::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::TransformExtent;
use crate::stream::Stream;
use crate::Transform;

use super::Reclip;
use super::ScaleSet;

impl<DRAIN, PR, T> ScaleSet for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>;
	type T = T;

	fn scale(mut self, scale: T) -> Self::Output {
		self.base.k = scale;
		self.reclip()
	}
}

impl<DRAIN, PR, T> ScaleSet for BuilderMercatorCircleResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Output = BuilderMercatorCircleResampleClip<DRAIN, PR, T>;
	type T = T;

	fn scale(mut self, scale: T) -> Self::Output {
		self.base.k = scale;
		self.reclip()
	}
}
