use std::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ReflectSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
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
		Resample<
			DRAIN,
			PR,
			NoClipC<DRAIN, T>,
			NoClipU<DRAIN, T>,
			ConnectedResample<NoClipC<DRAIN, T>, T>,
			T,
		>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Debug,
	LB: Debug,
	PR: Clone + Debug + Transform<T = T>,
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
		Resample<
			DRAIN,
			PR,
			ClipC<DRAIN, T>,
			ClipU<DRAIN, T>,
			ConnectedResample<ClipC<DRAIN, T>, T>,
			T,
		>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Debug,
	LB: Debug,
	PR: Clone + Debug + Transform<T = T>,
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
impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> ReflectSet
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		PCNC,
		PCNU,
		PR,
		PV,
		ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
		ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	> where
	DRAIN: Debug,
	PCNC: Debug,
	PCNU: Debug,
	PR: Clone + Debug + Transform<T = T>,
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
