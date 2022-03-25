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
use crate::clip::clip::Clip;
use crate::identity::Identity;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::ClipAngleSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;
use super::builder::Builder;
use super::ProjectionRawBase;
use super::Scale;

/// Defines a projection.
#[derive(Clone, Copy, Debug)]
pub struct Mercator<DRAIN, T> {
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Mercator<DRAIN, T>
where
	T: CoordFloat + FloatConst,
{
	#[inline]
	fn default() -> Self {
		Mercator {
			p_drain: PhantomData::<DRAIN>,
			p_t: PhantomData::<T>,
		}
	}
}

impl<DRAIN, T> ProjectionRawBase<T> for Mercator<DRAIN, T>
where
	DRAIN: Stream<EP = DRAIN, T = T> + Default,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Builder = Builder<
		DRAIN,
		InterpolateCircle<DRAIN, ResampleNoClipC<DRAIN, Mercator<DRAIN, T>, T>, T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoClipC<DRAIN, Mercator<DRAIN, T>, T>,
			Connected<ResampleNoClipC<DRAIN, Mercator<DRAIN, T>, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoClipC<DRAIN, Mercator<DRAIN, T>, T>, Unconnected, T>,
		Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
		Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
		Mercator<DRAIN, T>,
		PVCircle<T>,
		ResampleNoClipC<DRAIN, Mercator<DRAIN, T>, T>,
		ResampleNoClipU<DRAIN, Mercator<DRAIN, T>, T>,
		T,
	>;
	type T = T;

	#[inline]
	fn builder() -> Self::Builder
	where
		DRAIN: Stream<EP = DRAIN, T = T> + Default,
	{
		let clip = gen_clip_antimeridian::<
			DRAIN,
			NoClipC<DRAIN, T>,
			NoClipU<DRAIN, T>,
			Mercator<DRAIN, T>,
			ResampleNoClipC<DRAIN, Mercator<DRAIN, T>, T>,
			ResampleNoClipU<DRAIN, Mercator<DRAIN, T>, T>,
			T,
		>();
		Builder::new(clip, Mercator::default())
			.scale(T::from(250_f64).unwrap())
			.clip_angle(T::from(142_f64).unwrap())
	}
}

// impl<DRAIN, T> ProjectionRawMercator<T> for Mercator<DRAIN, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// }

// impl<DRAIN, T> TransformExtent<T> for Mercator<DRAIN, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
//     type T = T;

//     #[inline]
//     fn transform_extent(
//         self,
//         k: T,
//         t: Coordinate<T>,
//         _x0: T,
//         y0: T,
//         x1: T,
//         y1: T,
//     ) -> [Coordinate<T>; 2] {
//         [
//             Coordinate {
//                 x: Float::max(t.x - k, t.y - k),
//                 y: y0,
//             },
//             Coordinate {
//                 x: Float::min(t.x + k, x1),
//                 y: y1,
//             },
//         ]
//     }
// }

impl<DRAIN, EP, T> Transform for Mercator<DRAIN, T>
where
	DRAIN: Stream<EP = EP, T = T>,
	T: CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
		let two = T::from(2).unwrap();
		// Divergence between f64 and f32
		// when p.y  = 1.5707963267948966  (PI/2)
		// f64 outputs -37.33185619326892 which is consistent
		// with JS.
		// The f32 is different from JS. Technically
		// tan(PI/2) is NAN. and so log(NAN) is NAN.
		// The value returned
		// from tan(PI_f64/2_f64) happens to be the same
		// large number in both the JS and RUST.
		Coordinate {
			x: p.x,
			y: ((T::FRAC_PI_2() + p.y) / two).tan().ln(),
		}
	}

	#[inline]
	fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
		let two = T::from(2).unwrap();
		Coordinate {
			x: p.x,
			y: two * (p.y.exp()).atan() - T::FRAC_PI_2(),
		}
	}
}
