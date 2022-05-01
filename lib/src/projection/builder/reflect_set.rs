use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::types::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::ResampleClipC;
use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::ReflectSet;
use crate::Transform;

use super::template::ClipC;
use super::template::ClipU;
use super::template::NoClipC;
use super::template::NoClipU;
use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PV,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone + Transform<T = T>,
	T: CoordFloat + FloatConst,
{
	type T = T;

	/// Set the projection builder to invert the x-coordinate.
	fn reflect_x(mut self, reflect: bool) -> Self {
		if reflect {
			self.sx = T::from(-1.0_f64).unwrap();
		} else {
			self.sx = T::one();
		}
		self.recenter_with_resampling()
	}

	/// Set the projection builder to invert the y-coordinate.
	#[inline]
	fn reflect_y(mut self, reflect: bool) -> Self {
		if reflect {
			self.sy = T::from(-1.0_f64).unwrap();
		} else {
			self.sy = T::one();
		}
		self.recenter_with_resampling()
	}
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PV,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone + Transform<T = T>,
	T: CoordFloat + FloatConst,
{
	type T = T;

	/// Set the projection builder to invert the x-coordinate.
	fn reflect_x(mut self, reflect: bool) -> Self {
		if reflect {
			self.sx = T::from(-1.0_f64).unwrap();
		} else {
			self.sx = T::one();
		}
		self.recenter_with_resampling()
	}

	/// Set the projection builder to invert the y-coordinate.
	#[inline]
	fn reflect_y(mut self, reflect: bool) -> Self {
		if reflect {
			self.sy = T::from(-1.0_f64).unwrap();
		} else {
			self.sy = T::one();
		}
		self.recenter_with_resampling()
	}
}

// TODO must split itnto NoClip / Clip
impl<DRAIN, PR, T> ReflectSet for BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>
where
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: CoordFloat + FloatConst,
{
	type T = T;

	/// Set the projection builder to invert the x-coordinate.
	fn reflect_x(mut self, reflect: bool) -> Self {
		if reflect {
			self.sx = T::from(-1.0_f64).unwrap();
		} else {
			self.sx = T::one();
		}
		self.recenter_no_resampling()
	}

	/// Set the projection builder to invert the y-coordinate.
	#[inline]
	fn reflect_y(mut self, reflect: bool) -> Self {
		if reflect {
			self.sy = T::from(-1.0_f64).unwrap();
		} else {
			self.sy = T::one();
		}
		self.recenter_no_resampling()
	}
}
