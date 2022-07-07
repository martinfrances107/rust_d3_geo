use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::bounds::Bounds;
use crate::projection::builder::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::BuilderCircleResampleNoClip;
use crate::projection::builder::BuilderCircleResampleNoneNoClip;
use crate::projection::fit_no_clip::fit_extent_no_clip;
use crate::projection::fit_no_clip::fit_height_no_clip;
use crate::projection::fit_no_clip::fit_size_no_clip;
use crate::projection::fit_no_clip::fit_width_no_clip;
use crate::projection::Fit;
use crate::stream::Streamable;
use crate::Transform;

impl<PR, T> Fit for BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_extent_no_clip(self, extent, object)
	}

	#[inline]
	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_height_no_clip(self, h, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_size_no_clip(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_width_no_clip(self, w, object)
	}
}

impl<PR, T> Fit for BuilderCircleResampleNoneNoClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_extent_no_clip(self, extent, object)
	}

	#[inline]
	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_height(self, h, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_size(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_width(self, w, object)
	}
}

impl<PR, T> Fit for BuilderCircleResampleNoClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_extent_no_clip(self, extent, object)
	}

	#[inline]
	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_height(self, h, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_size_no_clip(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_width(self, w, object)
	}
}
