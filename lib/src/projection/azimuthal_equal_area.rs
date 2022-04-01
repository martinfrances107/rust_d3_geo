use crate::projection::ScaleSet;
use crate::stream::Stream;
use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::identity::Identity;
use crate::math::asin;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::Builder;
use crate::projection::ProjectionRawBase;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::azimuthal::azimuthal_raw;
use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;
use super::resampler::resample::Connected as ConnectedResample;
use super::resampler::resample::Resample;
use super::ClipAngleSet;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct AzimuthalEqualArea<DRAIN, T>
where
	T: CoordFloat + FloatConst,
{
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

// impl<DRAIN, T> ProjectionRawCommon<T> for AzimuthalEqualArea<DRAIN, T>
// where
//     DRAIN: Default + Stream<EP = DRAIN, T = T> ,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// }

impl<DRAIN, T> ProjectionRawBase<T> for AzimuthalEqualArea<DRAIN, T>
where
	DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Builder = Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			Resample<
				DRAIN,
				AzimuthalEqualArea<DRAIN, T>,
				Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
				Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
				ConnectedResample<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>, T>,
				T,
			>,
			Connected<
				Resample<
					DRAIN,
					AzimuthalEqualArea<DRAIN, T>,
					Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
					Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
					ConnectedResample<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>, T>,
					T,
				>,
			>,
			T,
		>,
		LineCircle<
			DRAIN,
			Resample<
				DRAIN,
				AzimuthalEqualArea<DRAIN, T>,
				Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
				Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
				ConnectedResample<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>, T>,
				T,
			>,
			Unconnected,
			T,
		>,
		Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
		Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
		AzimuthalEqualArea<DRAIN, T>,
		PVCircle<T>,
		Resample<
			DRAIN,
			AzimuthalEqualArea<DRAIN, T>,
			Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
			Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
			ConnectedResample<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>, T>,
			T,
		>,
		Resample<
			DRAIN,
			AzimuthalEqualArea<DRAIN, T>,
			Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
			Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
			Unconnected,
			T,
		>,
		T,
	>;
	type T = T;

	#[inline]
	fn builder() -> Self::Builder {
		let clip = gen_clip_antimeridian::<
			DRAIN,
			NoClipC<DRAIN, T>,
			NoClipU<DRAIN, T>,
			AzimuthalEqualArea<DRAIN, T>,
			ResampleNoClipC<DRAIN, AzimuthalEqualArea<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, AzimuthalEqualArea<DRAIN, T>, T>,
			T,
		>();

		Builder::new(clip, AzimuthalEqualArea::default())
			.scale(T::from(124.75_f64).unwrap())
			.clip_angle(T::from(180_f64 - 1e-3).unwrap())
	}
}

impl<DRAIN, T> Default for AzimuthalEqualArea<DRAIN, T>
where
	T: CoordFloat + FloatConst,
{
	fn default() -> Self {
		AzimuthalEqualArea {
			p_drain: PhantomData::<DRAIN>,
			p_t: PhantomData::<T>,
		}
	}
}

impl<DRAIN, T> AzimuthalEqualArea<DRAIN, T>
where
	T: CoordFloat + FloatConst,
{
	#[inline]
	fn cxcy(cxcy: T) -> T {
		(T::from(2).unwrap() / (T::one() + cxcy)).sqrt()
	}

	#[inline]
	fn z(z: T) -> T {
		let two = T::from(2.0_f64).unwrap();
		two * asin(z / two)
	}
}

impl<DRAIN, T> Transform for AzimuthalEqualArea<DRAIN, T>
where
	T: CoordFloat + FloatConst,
{
	type T = T;
	#[inline]
	fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
		azimuthal_raw(p, Self::cxcy)
	}

	#[inline]
	fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
		azimuthal_invert(p, Self::z)
	}
}
