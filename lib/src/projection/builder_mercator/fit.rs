use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::bounds::Bounds;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::Fit;
use crate::stream::Streamable;
use crate::Transform;

impl<PR, T> Fit for BuilderMercatorAntimeridianResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Debug + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	/// f32 or f64
	type T = T;

	#[inline]
	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_extent_clip(self, extent, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_size_adjust(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_width_adjust(self, w, object)
	}

	#[inline]
	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_height_adjust(self, h, object)
	}
}
