use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder_mercator::Builder;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::CenterSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

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
	PR: Clone + Transform<T = T>,
	T: CoordFloat + FloatConst,
{
	type T = T;

	fn center(mut self, center: &Coordinate<T>) -> Self {
		self.base = self.base.center(center);
		self
		// self.reclip()
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
	PR: Clone + Transform<T = T>,
	T: CoordFloat + FloatConst,
{
	type T = T;

	fn center(mut self, center: &Coordinate<T>) -> Self {
		self.base = self.base.center(center);
		self
	}
}
