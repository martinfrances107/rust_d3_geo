use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimerdian;
use crate::clip::buffer::Buffer;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder_mercator::ScaleSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder_mercator::Builder as MercatorBuilder;
use super::ProjectionRawBase;
use super::TransformExtent;

/// Defines a projection.
#[derive(Clone, Copy, Debug)]
pub struct Mercator<DRAIN, T> {
	p_drain: PhantomData<DRAIN>,
	p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Mercator<DRAIN, T> {
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
	DRAIN: 'static + Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Builder = MercatorBuilder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleClipC<DRAIN, Mercator<DRAIN, T>, T>,
			Connected<ResampleClipC<DRAIN, Mercator<DRAIN, T>, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleClipC<DRAIN, Mercator<DRAIN, T>, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		Mercator<DRAIN, T>,
		PVAntimerdian<T>,
		ResampleClipC<DRAIN, Mercator<DRAIN, T>, T>,
		ResampleClipU<DRAIN, Mercator<DRAIN, T>, T>,
		T,
	>;
	type T = T;

	#[inline]
	fn builder() -> Self::Builder {
		MercatorBuilder::new(Mercator::default()).scale(T::from(961_f64 / f64::TAU()).unwrap())
	}
}

// impl<DRAIN, T> ProjectionRawMercator<T> for Mercator<DRAIN, T>
// where
//     DRAIN: Default + Stream<EP = DRAIN, T = T> ,
//     T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// }

impl<DRAIN, T> TransformExtent<T> for Mercator<DRAIN, T>
where
	DRAIN: Clone,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn transform_extent(
		self,
		k: T,
		t: Coordinate<T>,
		_x0: T,
		y0: T,
		x1: T,
		y1: T,
	) -> [Coordinate<T>; 2] {
		[
			Coordinate {
				x: T::max(t.x - k, t.y - k),
				y: y0,
			},
			Coordinate {
				x: T::min(t.x + k, x1),
				y: y1,
			},
		]
	}
}

impl<DRAIN, T> Transform for Mercator<DRAIN, T>
where
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
