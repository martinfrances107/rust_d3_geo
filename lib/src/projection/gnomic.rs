use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::ClipAngleSet;
use crate::projection::ScaleAdjust;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;
use super::builder::Builder;
use super::ProjectionRawBase;

/// Gnomic
#[derive(Clone, Debug)]
pub struct Gnomic<DRAIN, T> {
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Gnomic<DRAIN, T> {
	fn default() -> Self {
		Gnomic {
			p_drain: PhantomData::<DRAIN>,
			p_t: PhantomData::<T>,
		}
	}
}

// impl<DRAIN, T> ProjectionRawCommon<T> for Gnomic<DRAIN, T>
// where
//     DRAIN: Default + Stream<EP = DRAIN, T = T> ,
//     // RESAMPLER: Resampler,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// }

impl<DRAIN, T> ProjectionRawBase<T> for Gnomic<DRAIN, T>
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
			ResampleNoClipC<DRAIN, Gnomic<DRAIN, T>, T>,
			Connected<ResampleNoClipC<DRAIN, Gnomic<DRAIN, T>, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoClipC<DRAIN, Gnomic<DRAIN, T>, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		Gnomic<DRAIN, T>,
		PVCircle<T>,
		ResampleNoClipC<DRAIN, Gnomic<DRAIN, T>, T>,
		ResampleNoClipU<DRAIN, Gnomic<DRAIN, T>, T>,
		T,
	>;

	/// f64 or f32.
	type T = T;

	fn builder() -> Self::Builder
	where
		DRAIN: Default + Stream<EP = DRAIN, T = T>,
	{
		let clip = gen_clip_antimeridian::<
			DRAIN,
			NoClipC<DRAIN, T>,
			NoClipU<DRAIN, T>,
			Gnomic<DRAIN, T>,
			ResampleNoClipC<DRAIN, Gnomic<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, Gnomic<DRAIN, T>, T>,
			T,
		>();
		Builder::new(clip, Gnomic::default())
			.scale(T::from(144.049_f64).unwrap())
			.clip_angle(T::from(60_f64).unwrap())
	}
}

impl<DRAIN, EP, T> Transform for Gnomic<DRAIN, T>
where
	DRAIN: Stream<EP = EP, T = T>,
	EP: Stream<EP = EP, T = T> + Default,
	T: CoordFloat + FloatConst,
{
	/// f64 or f32.
	type T = T;

	fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
		let cy = p.y.cos();
		let k = p.x.cos() * cy;
		Coordinate {
			x: cy * p.x.sin() / k,
			y: p.y.sin() / k,
		}
	}

	#[inline]
	fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
		azimuthal_invert(p, T::atan)
	}
}
