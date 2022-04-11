use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::TranslateSet;
use crate::projection::builder_mercator::ReclipAdjust;
use crate::stream::Connected;
use crate::stream::Stream;
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
	DRAIN: 'static + Clone + Default + Debug + Stream<EP=DRAIN, T=T>,
	I: Clone, LB: Clone + Debug, LC: Clone, LU:Clone,PCNC: Clone + Debug,
	PCNU: Clone + Debug,
	PR: Clone + Debug + Transform<T = T>,
	PV: Clone,
	T: 'static + AbsDiffEq<Epsilon=T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate(mut self, t: &Coordinate<T>) -> Self {
		self.base = self.base.translate(t);
		 self.reclip_adjust()

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
	DRAIN: 'static + Clone + Default + Debug + Stream<EP=DRAIN, T=T>,
	I: Clone, LB: Clone + Debug, LC: Clone, LU:Clone,PCNC: Clone + Debug,
	PCNU: Clone + Debug,
	PR: Clone + Debug + Transform<T = T>,
	PV: Clone,
	T: 'static + AbsDiffEq<Epsilon=T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate(mut self, t: &Coordinate<T>) -> Self {
		self.base = self.base.translate(t);
		self.reclip_adjust()


	}
}
