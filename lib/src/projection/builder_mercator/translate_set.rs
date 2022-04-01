use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::TranslateSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> TranslateSet
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
	T: CoordFloat + FloatConst,
{
	type T = T;

	fn translate(mut self, t: &Coordinate<T>) -> Self {
		self.base = self.base.translate(t);
		// self.reclip()
		self
	}
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> TranslateSet
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

	fn translate(mut self, t: &Coordinate<T>) -> Self {
		self.base = self.base.translate(t);
		// self.reclip()
		self
	}
}
