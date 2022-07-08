use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::Bufferable;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::path::bounds::Bounds;
use crate::projection::builder::Buffer;
use crate::projection::builder::Builder;
use crate::projection::builder::NoClipC;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::builder::ResampleNoneNoClipC;
use crate::projection::builder::ResampleNoneNoClipU;
use crate::projection::fit_no_clip::fit_extent_no_clip;
use crate::projection::fit_no_clip::fit_height_no_clip;
use crate::projection::fit_no_clip::fit_size_no_clip;
use crate::projection::fit_no_clip::fit_width_no_clip;
use crate::projection::Debug;
use crate::projection::Fit;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

impl<I, LC, LB, LU, PR, PV, T> Fit
	for Builder<
		Bounds<T>,
		I,
		LB,
		LC,
		LU,
		NoClipC<Bounds<T>, T>,
		NoClipU<Bounds<T>, T>,
		PR,
		PV,
		ResampleNoneNoClipC<Bounds<T>, PR, T>,
		ResampleNoneNoClipU<Bounds<T>, PR, T>,
		T,
	> where
	I: Clone + Interpolator<T = T>,
	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone
		+ LineConnected<SC = ResampleNoneNoClipC<Bounds<T>, PR, T>>
		+ Stream<EP = Bounds<T>, T = T>,
	LU: Clone
		+ Connectable<Output = LC, SC = ResampleNoneNoClipC<Bounds<T>, PR, T>>
		+ Bufferable<Output = LB, T = T>
		+ Debug,
	PR: Clone + Transform<T = T>,
	PV: Clone + PointVisible<T = T>,
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

impl<I, LC, LB, LU, PR, PV, T> Fit
	for Builder<
		Bounds<T>,
		I,
		LB,
		LC,
		LU,
		NoClipC<Bounds<T>, T>,
		NoClipU<Bounds<T>, T>,
		PR,
		PV,
		ResampleNoClipC<Bounds<T>, PR, T>,
		ResampleNoClipU<Bounds<T>, PR, T>,
		T,
	> where
	I: Clone + Interpolator<T = T>,
	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone
		+ LineConnected<SC = ResampleNoClipC<Bounds<T>, PR, T>>
		+ Stream<EP = Bounds<T>, T = T>,
	LU: Clone
		+ Connectable<Output = LC, SC = ResampleNoClipC<Bounds<T>, PR, T>>
		+ Bufferable<Output = LB, T = T>
		+ Debug,
	PR: Clone + Transform<T = T>,
	PV: Clone + PointVisible<T = T>,
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
