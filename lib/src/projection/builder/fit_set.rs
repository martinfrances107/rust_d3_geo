use crate::path::bounds::Bounds;
use crate::projection::builder::BuilderAntimeridianResampleClip;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder::BuilderAntimeridianResampleNoClip;
use crate::projection::fit_no_rectangle::fit_extent;
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
	// DRAIN: Clone + Debug + Default + PartialEq + Stream<EP = DRAIN, T = T>,
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
		fit_extent(self, extent, object)
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
// impl<DRAIN, I, LB, LC, LU, PR, PV, T> FitSet
// 	for Builder<
// 		DRAIN,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		NoClipC<DRAIN, T>,
// 		NoClipU<DRAIN, T>,
// 		PR,
// 		PV,
// 		ResampleNoClipC<DRAIN, PR, T>,
// 		ResampleNoClipU<DRAIN, PR, T>,
// 		T,
// 	> where
// 	I: Clone,
// 	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
// 	LC: Clone,
// 	PV: Clone + Debug,
// 	PR: Transform<T = T>,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	type Output = Builder<
// 		DRAIN,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<DRAIN, T>,
// 		ClipU<DRAIN, T>,
// 		PR,
// 		PV,
// 		ResampleClipC<DRAIN, PR, T>,
// 		ResampleClipU<DRAIN, PR, T>,
// 		T,
// 	>;
// 	type T = T;

// 	#[inline]
// 	fn fit_extent(
// 		self,
// 		extent: [[T; 2]; 2],
// 		object: &impl Streamable<T = Self::T>,
// 	) -> Self::Output {
// 		fit_extent(self, extent, object)
// 	}

// 	#[inline]
// 	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		todo!();
// 		// fit_height(self, h, object)
// 	}

// 	#[inline]
// 	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		todo!();
// 		// fit_size(self, size, object)
// 	}

// 	#[inline]
// 	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		todo!();
// 		// fit_width(self, w, object)
// 	}
// }
