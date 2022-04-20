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
use crate::math::acos;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::Builder;
use crate::projection::ClipAngleSet;
use crate::projection::ProjectionRawBase;
use crate::projection::ScaleAdjust;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::azimuthal::azimuthal_raw;
use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct AzimuthalEquiDistant<DRAIN, T> {
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

// impl<DRAIN, T> ProjectionRawCommon<T> for AzimuthalEquiDistant<DRAIN, T>
// where
//     DRAIN: Default + Stream<EP = DRAIN, T = T> ,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// }

impl<DRAIN, T> ProjectionRawBase<T> for AzimuthalEquiDistant<DRAIN, T>
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
			ResampleNoClipC<DRAIN, AzimuthalEquiDistant<DRAIN, T>, T>,
			Connected<ResampleNoClipC<DRAIN, AzimuthalEquiDistant<DRAIN, T>, T>>,
			T,
		>,
		LineCircle<
			DRAIN,
			ResampleNoClipC<DRAIN, AzimuthalEquiDistant<DRAIN, T>, T>,
			Unconnected,
			T,
		>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		AzimuthalEquiDistant<DRAIN, T>,
		PVCircle<T>,
		ResampleNoClipC<DRAIN, AzimuthalEquiDistant<DRAIN, T>, T>,
		ResampleNoClipU<DRAIN, AzimuthalEquiDistant<DRAIN, T>, T>,
		T,
	>;
	type T = T;

	#[inline]
	fn builder() -> Self::Builder {
		let clip = gen_clip_antimeridian::<
			DRAIN,
			NoClipC<DRAIN, T>,
			NoClipU<DRAIN, T>,
			AzimuthalEquiDistant<DRAIN, T>,
			ResampleNoClipC<DRAIN, AzimuthalEquiDistant<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, AzimuthalEquiDistant<DRAIN, T>, T>,
			T,
		>();
		Builder::new(clip, AzimuthalEquiDistant::default())
			.scale(T::from(79.4188_f64).unwrap())
			.clip_angle(T::from(180_f64 - 1e-3).unwrap())
	}
}

impl<DRAIN, T> Default for AzimuthalEquiDistant<DRAIN, T>
where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,
	T: CoordFloat + FloatConst,
{
	fn default() -> Self {
		AzimuthalEquiDistant {
			p_drain: PhantomData::<DRAIN>,
			p_t: PhantomData::<T>,
		}
	}
}

impl<DRAIN, T> AzimuthalEquiDistant<DRAIN, T>
where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,
	T: CoordFloat + FloatConst,
{
	#[inline]
	fn c(c: T) -> T {
		let c = acos(c);
		if c == T::zero() {
			c
		} else {
			c / c.sin()
		}
	}

	#[inline]
	fn z(z: T) -> T {
		z
	}
}

impl<DRAIN, T> Transform for AzimuthalEquiDistant<DRAIN, T>
where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,
	T: CoordFloat + FloatConst,
{
	type T = T;
	#[inline]
	fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
		azimuthal_raw(p, Self::c)
	}

	#[inline]
	fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
		azimuthal_invert(p, Self::z)
	}
}
