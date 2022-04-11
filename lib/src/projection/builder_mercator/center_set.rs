use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::CenterSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
use super::ReclipAdjust;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> CenterSet
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
	DRAIN: 'static + Clone + Default + Debug + Stream<EP=DRAIN,T=T>,
	I: Clone,
	LB: Clone + Debug,
	LC: Clone,
	LU: Clone,
	PCNC: Clone + Debug,
	PCNU: Clone + Debug,
	PR: Clone + Debug + Transform<T = T>,
	PV: Clone,
	T: 'static + AbsDiffEq<Epsilon=T> + CoordFloat + FloatConst,
{
	type T = T;

	fn center(mut self, center: &Coordinate<T>) -> Self {
		self.base = self.base.center(center);
		// self
		self.reclip_adjust()
	}
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> CenterSet
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
	DRAIN: 'static + Clone + Default + Debug + Stream<EP=DRAIN,T=T>,
	I: Clone,
	LB: Clone + Debug,
	LC: Clone,
	LU: Clone,
	PCNC: Clone + Debug,
	PCNU: Clone + Debug,
	PR: Clone + Debug + Transform<T = T>,
	PV: Clone,
	T: 'static + AbsDiffEq<Epsilon=T> + CoordFloat + FloatConst,
{
	type T = T;

	fn center(mut self, center: &Coordinate<T>) -> Self {
		self.base = self.base.center(center);
		self.reclip_adjust()
	}
}
