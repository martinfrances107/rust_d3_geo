use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::Reflect;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Reflect
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
		Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
		Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	> where
	DRAIN: Clone + Debug,
	I: Clone,
	LB: Clone,
	LC: Clone + Debug,
	LU: Clone + Debug,
	PV: Clone + Debug,
	PCNC: Clone + Debug,
	PCNU: Clone + Debug,
	PR: Transform<T = T>,
	PR: Clone + Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Is the projection builder set to invert the x-coordinate.
	#[inline]
	fn get_reflect_x(&self) -> bool {
		self.sx < T::zero()
	}

	/// Set the projection builder to invert the x-coordinate.
	fn reflect_x(mut self, reflect: bool) -> Self {
		if reflect {
			self.sx = T::from(-1.0_f64).unwrap();
		} else {
			self.sx = T::one();
		}
		self.recenter_with_resampling()
	}

	/// Is the projection builder set to invert the y-coordinate.
	#[inline]
	fn get_reflect_y(&self) -> bool {
		self.sy < T::zero()
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

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Reflect
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
	PR: Clone + Transform<T = T>,
	PV: Clone + Debug,
	DRAIN: Clone + Debug,
	PCNU: Clone + Debug,
	LB: Clone,
	LU: Debug,
	LC: Clone + Debug,
	I: Clone,
	LU: Clone,
	T: 'static + AsPrimitive<T> + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Is the projection builder set to invert the x-coordinate.
	#[inline]
	fn get_reflect_x(&self) -> bool {
		self.sx < T::zero()
	}

	/// Set the projection builder to invert the x-coordinate.
	fn reflect_x(mut self, reflect: bool) -> Self {
		if reflect {
			self.sx = T::from(-1.0_f64).unwrap();
		} else {
			self.sx = T::one();
		}
		self.recenter_no_resampling()
	}

	/// Is the projection builder set to invert the y-coordinate.
	#[inline]
	fn get_reflect_y(&self) -> bool {
		self.sy < T::zero()
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
