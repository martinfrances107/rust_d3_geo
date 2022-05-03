// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::clip::rectangle::Rectangle;
// use crate::path::bounds::Bounds;
// use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
// use crate::projection::FitAdjust;
// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Streamable;
// use crate::stream::Unconnected;
// use crate::Transform;

// use super::Builder;

// impl<DRAIN, PR, T> FitAdjust for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
// where
// 	DRAIN: Stream<EP = DRAIN, T = T>,
// 	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// 	/// f32 or f64
// 	type T = T;

// 	#[inline]
// 	fn fit_extent_adjust(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		// fit_extent_adjust(self, extent, object)
// 	}

// 	#[inline]
// 	fn fit_size_adjust(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		todo!();
// 		// fit_size_adjust(self, size, object)
// 	}

// 	#[inline]
// 	fn fit_width_adjust(self, w: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		todo!();
// 		// fit_width_adjust(self, w, object)
// 	}

// 	#[inline]
// 	fn fit_height_adjust(self, h: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		todo!();
// 		// fit_height_adjust(self, h, object)
// 	}
// }
