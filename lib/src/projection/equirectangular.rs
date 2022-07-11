use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::Scale;
use crate::stream::Stream;
use crate::Transform;

use super::builder::template::NoClipC;
use super::builder::template::NoClipU;
use super::builder::Builder;
use super::ProjectionRawBase;

/// Equirectangular
/// Used to define a projection builder.
#[derive(Clone, Copy, Debug)]
pub struct Equirectangular<DRAIN, T> {
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Equirectangular<DRAIN, T> {
	fn default() -> Self {
		Self {
			p_drain: PhantomData::<DRAIN>,
			p_t: PhantomData::<T>,
		}
	}
}

impl<DRAIN, T> ProjectionRawBase<T> for Equirectangular<DRAIN, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	T: CoordFloat + FloatConst,
{
	type Builder = BuilderAntimeridianResampleNoClip<DRAIN, Equirectangular<DRAIN, T>, T>;
	type T = T;

	#[inline]
	fn builder() -> Self::Builder {
		let clip = gen_clip_antimeridian::<_, NoClipC<DRAIN>, NoClipU<DRAIN>, _, _, _, _>();
		Builder::new(clip, Equirectangular::default()).scale(T::from(152.63_f64).unwrap())
	}
}

impl<DRAIN, T> Transform for Equirectangular<DRAIN, T>
where
	T: CoordFloat,
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
