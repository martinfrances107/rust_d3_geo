// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::clip::rectangle::Rectangle;
// use crate::path::bounds::Bounds;
// use crate::projection::fit::fit_extent_adjust;
// use crate::projection::fit::fit_height_adjust;
// use crate::projection::fit::fit_size_adjust;
// use crate::projection::fit::fit_width_adjust;
// use crate::projection::FitAdjust;
// use crate::stream::Connected;
// use crate::stream::Streamable;
// use crate::stream::Unconnected;
// use crate::Transform;

// use super::Builder;

// impl<I, LB, LC, LU, PR, PV, RC, RU, T> FitAdjust
// 	for Builder<
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
// 	> where
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// 	/// f32 or f64
// 	type T = T;

// 	#[inline]
// 	fn fit_extent_adjust(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_extent_adjust(self, extent, object)
// 	}

// 	#[inline]
// 	fn fit_size_adjust(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_size_adjust(self, size, object)
// 	}

// 	#[inline]
// 	fn fit_width_adjust(self, w: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_width_adjust(self, w, object)
// 	}

// 	#[inline]
// 	fn fit_height_adjust(self, h: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		fit_height_adjust(self, h, object)
// 	}
// }
