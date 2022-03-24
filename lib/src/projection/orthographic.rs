use crate::clip::clip::Clip;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleNoneNoClipU;
use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::identity::Identity;
use crate::math::asin;
use crate::math::EPSILON;
// use crate::projection::builder::template::ResampleNoClipC;
// use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::resampler::none::None;
use crate::projection::ClipAngleSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder::Builder;
use super::ProjectionRawBase;
use super::Scale;
use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;

/// Orthographic
///
/// Root transform.
/// Used to define a projection builder.
#[derive(Clone, Copy, Debug)]
pub struct Orthographic<DRAIN, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     T: CoordFloat + FloatConst,
{
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Orthographic<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,
	T: CoordFloat + FloatConst,
{
	fn default() -> Self {
		Orthographic {
			p_drain: PhantomData::<DRAIN>,
			p_t: PhantomData::<T>,
		}
	}
}

// impl<DRAIN, T> ProjectionRawCommon<T> for Orthographic<DRAIN, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// }

impl<DRAIN, T> ProjectionRawBase<T> for Orthographic<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Builder = Builder<
		DRAIN,
		InterpolateCircle<
			DRAIN,
			ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
			T,
		>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
			Connected<
				ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
			>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>, Unconnected, T>,
		Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
		Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
		Orthographic<DRAIN, T>,
		PVCircle<T>,
		ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
		ResampleNoClipU<DRAIN, Orthographic<DRAIN, T>, T>,
		T,
	>;

	type T = T;
	#[inline]
	fn builder() -> Self::Builder {
		let clip: Clip<
			DRAIN,
			InterpolateAntimeridian<
				DRAIN,
				ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
				T,
			>,
			LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
			LineAntimeridian<
				DRAIN,
				ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
				Connected<ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>>,
				T,
			>,
			LineAntimeridian<
				DRAIN,
				ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
				Unconnected,
				T,
			>,
			Orthographic<DRAIN, T>,
			PVAntimeridian<T>,
			ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, Orthographic<DRAIN, T>, T>,
			Unconnected,
			T,
		> = gen_clip_antimeridian::<
			DRAIN,
			NoClipC<DRAIN, T>,
			NoClipU<DRAIN, T>,
			Orthographic<DRAIN, T>,
			ResampleNoClipC<DRAIN, Orthographic<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, Orthographic<DRAIN, T>, T>,
			T,
		>();
		Builder::new(clip, Orthographic::default())
			.scale(T::from(249.5_f64).unwrap())
			.clip_angle(T::from(90_f64 + EPSILON).unwrap())
	}
}

impl<DRAIN, T> Orthographic<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,
	T: CoordFloat + FloatConst,
{
	#[inline]
	fn angle(z: T) -> T {
		asin(z)
	}

	fn azimuthal_invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
		let z = (p.x * p.x + p.y * p.y).sqrt();
		let c = Orthographic::<DRAIN, T>::angle(z);
		let sc = c.sin();
		let cc = c.cos();

		let ret_x = (p.x * sc).atan2(z * cc);

		let y_out = if z == T::zero() { z } else { p.y * sc / z };
		let ret_y = asin(y_out);

		Coordinate { x: ret_x, y: ret_y }
	}
}

impl<DRAIN, T> Transform for Orthographic<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,

	T: CoordFloat + FloatConst,
{
	type T = T;
	#[inline]
	fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
		Coordinate {
			x: p.y.cos() * p.x.sin(),
			y: p.y.sin(),
		}
	}

	#[inline]
	fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
		self.azimuthal_invert(p)
	}
}
