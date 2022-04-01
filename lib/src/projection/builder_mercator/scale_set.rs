use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ScaleSet;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> ScaleSet
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
	I: Interpolator<T = T>,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Clean + Stream<EP = Buffer<T>, T = T>,
	LC: Clone
		+ LineConnected<SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>
		+ Stream<EP = DRAIN, T = T>,
	LU: Clone
		+ Debug
		+ Connectable<
			Output = LC,
			SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
		> + Bufferable<Output = LB, T = T>,
	PCNC: Debug,
	PCNU: Debug,
	PR: Clone + Debug + Transform<T = T>,
	PV: PointVisible<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn scale(mut self, scale: T) -> Self {
		self.base = self.base.scale(scale);
		// self.reclip()
		self
	}
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> ScaleSet
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
	I: Interpolator<T = T>,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Clean + Stream<EP = Buffer<T>, T = T>,
	LC: Clone
		+ LineConnected<SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
		+ Stream<EP = DRAIN, T = T>,
	LU: Clone
		+ Debug
		+ Connectable<Output = LC, SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
		+ Bufferable<Output = LB, T = T>,
	PCNC: Debug,
	PCNU: Debug,
	PR: Clone + Debug + Transform<T = T>,
	PV: PointVisible<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn scale(mut self, scale: T) -> Self {
		self.base = self.base.scale(scale);
		// self.reclip()
		self
	}
}
