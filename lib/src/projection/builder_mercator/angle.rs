use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder_mercator::Builder;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::AngleGet;
use crate::projection::AngleSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> AngleGet
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	// PCNU: Debug,
	// RU: Debug,
	T: CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn get_angle(&self) -> T {
		self.base.get_angle()
	}
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> AngleSet
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
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Sets the rotation angles as measured in degrees.
	fn angle(self, angle: T) -> Self {
		let base = self.base.angle(angle);
		Self {
			pr: self.pr,
			base,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,
		}
	}
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> AngleSet
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
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Sets the rotation angles as measured in degrees.
	fn angle(self, angle: T) -> Self {
		let base = self.base.angle(angle);
		Self {
			pr: self.pr,
			base,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,
		}
	}
}
