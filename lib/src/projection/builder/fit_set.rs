use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::Bufferable;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::path::bounds::Bounds;
use crate::projection::builder::Buffer;
use crate::projection::builder::ClipC;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipC;
use crate::projection::builder::NoClipU;
use crate::projection::fit_set::fit_extent;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::resampler::Resampler;
use crate::projection::FitAdjust;
use crate::projection::FitSet;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<I, LB, LC, LU, PR, PV, RC, RU, T> FitSet
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
		RC,
		RU,
		T,
	> where
	I: Clone,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone + Debug + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
	LU: Clone + Debug + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
	PR: Transform<T = T>,
	PV: Clone + Debug,
	RC: Clone + Debug + Stream<EP = Bounds<T>, T = T>,
	RU: Debug + Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>, T>>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = Builder<
		Bounds<T>,
		I,
		LB,
		LC,
		LU,
		ClipC<Bounds<T>, T>,
		ClipU<Bounds<T>, T>,
		PR,
		PV,
		RC,
		RU,
		T,
	>;
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
		fit_height(self, h, object)
	}

	#[inline]
	fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_size(self, size, object)
	}

	#[inline]
	fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self::Output
	where
		Self::T: AsPrimitive<T> + CoordFloat,
	{
		fit_width(self, w, object)
	}
}
