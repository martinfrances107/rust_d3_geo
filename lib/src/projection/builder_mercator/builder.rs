use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::clip::antimeridian::gen_clip_antimeridian;
// use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
// use crate::clip::antimeridian::line::Line as LineAntimeridian;
// use crate::clip::antimeridian::pv::PV as PVAntimeridian;
// use crate::clip::buffer::Buffer;
// use crate::clip::PointVisible;
// use crate::identity::Identity;
// use crate::projection::builder::template::ResampleNoClipC;
// use crate::projection::builder::template::ResampleNoClipU;
// use crate::projection::builder::Builder as ProjectionBuilder;

use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder_mercator::Builder as MercatorBuilder;
use crate::projection::ClipExtentSet;
use crate::rot::rotate_radians;
// use crate::projection::resampler::none::None as ResampleNone;
// use crate::projection::resampler::resample::Connected as ConnectedResample;
// use crate::projection::resampler::resample::Resample;
// use crate::projection::stream_transform_radians::StreamTransformRadians;
// use crate::projection::ClipExtentBounded;
// use crate::projection::Projector;
use crate::projection::TransformExtent;
// use crate::projection::TranslateGet;
// use crate::projection::TranslateSet;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
// use crate::stream::Connected;
// use crate::stream::Stream;
use crate::Coordinate;
use crate::Transform;

use super::Builder;

use crate::stream::Unconnected;

// impl<I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Fit
// 	for MercatorBuilder<Bounds<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
// 	I: Clone + Debug,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PCNC: Clone + Debug,
// 	PCNU: Clone + Debug,
// 	PR: TransformExtent<T>,
// 	PV: PointVisible<T = T>,
// 	RC: Clone + Debug,
// 	RU: Clone + Debug,
// 	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	type T = T;

// 	#[inline]
// 	fn fit_extent(mut self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_extent(extent, object);
// 		self
// 	}

// 	#[inline]
// 	fn fit_size(mut self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_size(size, object);
// 		self
// 	}

// 	#[inline]
// 	fn fit_width(mut self, w: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_width(w, object);
// 		self
// 	}

// 	/// Similar to fit_size where the width is automatically chosen from
// 	/// the aspect ratio of object and the given constraint on height.
// 	#[inline]
// 	fn fit_height(mut self, h: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_height(h, object);
// 		self
// 	}
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Reflect
// 	for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
// 	DRAIN: Default + Stream<EP = DRAIN, T = T>,
// 	PV: PointVisible<T = T>,

// 	T: 'static
// 		+ AbsDiffEq<Epsilon = T>
// 		+ std::ops::AddAssign
// 		+ AsPrimitive<T>
// 		+ CoordFloat
// 		// + Display
// 		+ FloatConst,
// {
// 	type T = T;

// 	/// Is the projection builder set to invert the x-coordinate.
// 	#[inline]
// 	fn get_reflect_x(&self) -> bool {
// 		self.base.get_reflect_x()
// 	}

// 	/// Set the projection builder to invert the x-coordinate.
// 	fn reflect_x(mut self, reflect: bool) -> Self {
// 		self.base = self.base.reflect_x(reflect);
// 		self
// 	}

// 	/// Is the projection builder set to invert the y-coordinate.
// 	#[inline]
// 	fn get_reflect_y(&self) -> bool {
// 		self.base.get_reflect_y()
// 	}

// 	/// Set the projection builder to invert the y-coordinate.
// 	#[inline]
// 	fn reflect_y(mut self, reflect: bool) -> Self {
// 		self.base = self.base.reflect_y(reflect);
// 		self
// 	}
// }
