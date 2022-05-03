use crate::path::bounds::Bounds;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoneClip;
use crate::projection::builder::BuilderAntimeridianResampleClip;
use crate::projection::builder::BuilderCircleResampleNoClip;
use crate::projection::builder::BuilderCircleResampleNoneNoClip;
use crate::projection::fit_no_rectangle::fit_extent_antimerdian;
use crate::projection::fit_no_rectangle::fit_extent_circle_none_no_clip;
use crate::projection::fit_no_rectangle::fit_extent_circle_resample_no_clip;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder::BuilderAntimeridianResampleNoClip;
use crate::projection::FitSet;
use crate::stream::Streamable;
use crate::Transform;

// impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> FitSet
// 	for Builder<
// 		DRAIN,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		NoClipC<Bounds<T>, T>,
// 		NoClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		ResampleNoneNoClipC<DRAIN, PR, T>,
// 		ResampleNoneNoClipU<DRAIN, PR, T>,
// 		T,
// 	>
// // where drain support RESULT - >[Coordinate;2]?
// {
// 	type Output = Builder<
// 		DRAIN,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		ResampleNoneClipC<DRAIN, PR, T>,
// 		ResampleNoneClipU<DRAIN, PR, T>,
// 		T,
// 	>;
// 	type T = T;

// 	#[inline]
// 	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_extent(self, extent, object)
// 	}

// 	#[inline]
// 	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_height(self, h, object)
// 	}

// 	#[inline]
// 	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_size(self, size, object)
// 	}

// 	#[inline]
// 	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_width(self, w, object)
// 	}
// }

// InterpolateAntimeridian<T>,
// LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
// LineAntimeridian<
// 	DRAIN,
// 	ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
// 	Connected<ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>>,
// 	T,
// >,
// LineAntimeridian<
// 	DRAIN,
// 	ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
// 	Unconnected,
// 	T,
// >,

impl<PR, T> FitSet for BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Output = BuilderAntimeridianResampleClip<Bounds<T>, PR, T>;
	type T = T;

	#[inline]
	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_extent_antimerdian(self, extent, object)
	}

	#[inline]
	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_height(self, h, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_size(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_width(self, w, object)
	}
}

impl<PR, T> FitSet for BuilderCircleResampleNoneNoClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Output = BuilderCircleResampleNoneClip<Bounds<T>, PR, T>;
	type T = T;

	#[inline]
	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_extent_circle_none_no_clip(self, extent, object)
	}

	#[inline]
	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_height(self, h, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_size(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_width(self, w, object)
	}
}

impl<PR, T> FitSet for BuilderCircleResampleNoClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Output = BuilderCircleResampleClip<Bounds<T>, PR, T>;
	type T = T;

	#[inline]
	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_extent_circle_resample_no_clip(self, extent, object)
	}

	#[inline]
	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_height(self, h, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_size(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		todo!();
		// fit_width(self, w, object)
	}
}
