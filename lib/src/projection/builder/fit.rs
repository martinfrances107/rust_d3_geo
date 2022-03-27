// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::clip::rectangle::Rectangle;
// // use crate::clip::Interpolate;
// use crate::clip::PointVisible;
// use crate::identity::Identity;
// use crate::path::bounds::Bounds;
// use crate::projection::fit::fit_extent_adjust;
// use crate::Transform;
// // use crate::projection::fit::fit_extent;
// // use crate::projection::fit::fit_height_adjust;
// // use crate::projection::fit::fit_height_convert;
// // use crate::projection::fit::fit_size_adjust;
// // use crate::projection::fit::fit_size_convert;
// // use crate::projection::fit::fit_width_adjust;
// // use crate::projection::fit::fit_width_convert;
// use crate::projection::resampler::resample::Connected as ConnectedResample;
// use crate::projection::resampler::resample::Resample;
// use crate::projection::resampler::Resampler;
// use crate::projection::Fit;
// // use crate::projection::Interpolate;
// use crate::stream::Connectable;

// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Streamable;
// use crate::stream::Unconnected;

// use super::Builder;

// use super::PostClipNode;
// use super::ProjectionRawBase;

// impl<I, LB, LC, LU, PR, PV, RC, RU, T> Fit
// 	for Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	> where
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// 	type T = T;
// 	type Output = Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>;
// 	#[inline]
// 	fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_extent(self, extent, object)
// 	}

// 	#[inline]
// 	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_size_convert(self, size, object)
// 	}

// 	#[inline]
// 	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_width_convert(self, w, object)
// 	}

// 	#[inline]
// 	fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self::Output
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_height_convert(self, h, object)
// 	}
// }
