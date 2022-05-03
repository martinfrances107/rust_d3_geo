use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoneClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoneNoClip;
use crate::projection::builder_mercator::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::TransformExtent;
use crate::stream::Stream;
use crate::Transform;

use super::Reclip;
use super::TranslateSet;

impl<DRAIN, PR, T> TranslateSet for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>;
	type T = T;

	#[inline]
	fn translate(mut self, t: &Coordinate<T>) -> Self::Output {
		self.base.x = t.x;
		self.base.y = t.y;
		self.reclip()
	}
}

impl<DRAIN, PR, T> TranslateSet for BuilderMercatorAntimeridianResampleNoneNoClip<DRAIN, PR, T>
where
	DRAIN: Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	T: AsPrimitive<T> + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>;

	#[inline]
	fn translate(mut self, t: &Coordinate<T>) -> Self::Output {
		self.base.x = t.x;
		self.base.y = t.y;
		self.reclip()
	}
}
