use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::clip::Clip;
use crate::identity::Identity;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;
use super::builder::Builder;
use super::ProjectionRawBase;
use super::Scale;

/// Equirectangular
/// Used to define a projection builder.
#[derive(Clone, Copy, Debug)]
pub struct Equirectangular<DRAIN, T> {
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Equirectangular<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,
	T: CoordFloat,
{
	fn default() -> Self {
		Self {
			p_drain: PhantomData::<DRAIN>,
			p_t: PhantomData::<T>,
		}
	}
}

// impl<DRAIN, T> ProjectionRawCommon<T> for Equirectangular<DRAIN, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// }

impl<DRAIN, T> ProjectionRawBase<T> for Equirectangular<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Builder = Builder<
		DRAIN,
		InterpolateAntimeridian<DRAIN, ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>, T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
			Connected<ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>>,
			T,
		>,
		LineAntimeridian<
			DRAIN,
			ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
			Unconnected,
			T,
		>,
		Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
		Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
		Equirectangular<DRAIN, T>,
		PVAntimeridian<T>,
		ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
		ResampleNoClipU<DRAIN, Equirectangular<DRAIN, T>, T>,
		T,
	>;
	type T = T;

	#[inline]
	fn builder() -> Self::Builder
	where
		DRAIN: Stream<EP = DRAIN, T = T> + Default,
	{
		let clip: Clip<
			DRAIN,
			InterpolateAntimeridian<DRAIN, ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>, T>,
			LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
			LineAntimeridian<
				DRAIN,
				ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
				Connected<ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>>,
				T,
			>,
			LineAntimeridian<
				DRAIN,
				ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
				Unconnected,
				T,
			>,
			Equirectangular<DRAIN, T>,
			PVAntimeridian<T>,
			ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, Equirectangular<DRAIN, T>, T>,
			Unconnected,
			T,
		> = gen_clip_antimeridian::<
			DRAIN,
			NoClipC<DRAIN, T>,
			NoClipU<DRAIN, T>,
			Equirectangular<DRAIN, T>,
			ResampleNoClipC<DRAIN, Equirectangular<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, Equirectangular<DRAIN, T>, T>,
			T,
		>();
		Builder::new(clip, Equirectangular::default()).scale(T::from(152.63_f64).unwrap())
	}
}

impl<DRAIN, T> Transform for Equirectangular<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,
	T: CoordFloat + FloatConst,
{
	/// f64 or f32.
	type T = T;

	fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
		*p
	}
	fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
		*p
	}
}
