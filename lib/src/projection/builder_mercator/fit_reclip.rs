use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::builder_mercator::FitReclip;
use crate::stream::Stream;
use crate::stream::Streamable;

impl<DRAIN, PR, T> FitReclip for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Stream<EP = DRAIN, T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	/// f32 or f64
	type T = T;
	type Output = BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>;

	#[inline]
	fn fit_extent_reclip(
		self,
		extent: [[T; 2]; 2],
		object: &impl Streamable<T = Self::T>,
	) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		// fit_extent_adjust(self, extent, object)
		todo!();
	}

	#[inline]
	fn fit_size_reclip(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_size_adjust(self, size, object)
	}

	#[inline]
	fn fit_width_reclip(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_width_adjust(self, w, object)
	}

	#[inline]
	fn fit_height_reclip(self, h: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_height_adjust(self, h, object)
	}
}
