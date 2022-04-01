use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::RotateSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> RotateSet
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
	DRAIN: Debug,
	LB: Debug,
	PCNC: Debug,
	PCNU: Debug,
	PR: Clone + Debug + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	/// Sets the rotation angles as measured in degrees.
	fn rotate(mut self, angles: &[T; 3]) -> Self {
		self.base = self.base.rotate(angles);
		self
	}
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> RotateSet
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
	LB: Debug,
	PCNC: Debug,
	PCNU: Debug,
	PR: Clone + Debug + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	/// Sets the rotation angles as measured in degrees.
	fn rotate(mut self, angles: &[T; 3]) -> Self {
		self.base = self.base.rotate(angles);
		self
	}
}
