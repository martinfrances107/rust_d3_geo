use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::CenterSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
use super::ReclipAdjust;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> CenterSet
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
		Resample<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Unconnected, T>,
		T,
	> where
	DRAIN: 'static + Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
	I: Clone + Interpolator<T = T>,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Clean + Stream<EP = Buffer<T>, T = T>,
	LC: Clone
		+ LineConnected<
			SC = Resample<
				DRAIN,
				PR,
				ClipC<DRAIN, T>,
				ClipU<DRAIN, T>,
				ConnectedResample<ClipC<DRAIN, T>, T>,
				T,
			>,
		> + Stream<EP = DRAIN, T = T>,
	LU: Clone
		+ Debug
		+ Connectable<
			Output = LC,
			SC = Resample<
				DRAIN,
				PR,
				ClipC<DRAIN, T>,
				ClipU<DRAIN, T>,
				ConnectedResample<ClipC<DRAIN, T>, T>,
				T,
			>,
		> + Bufferable<Output = LB, T = T>,
	PR: Clone + Debug + Transform<T = T>,
	PV: Clone + PointVisible<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + Debug + FloatConst,
{
	type T = T;

	fn center(mut self, center: &Coordinate<T>) -> Self {
		self.base = self.base.center(center);
		// self
		self.reclip_adjust()
	}
}

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> CenterSet
// 	for Builder<
// 		DRAIN,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		PCNC,
// 		PCNU,
// 		PR,
// 		PV,
// 		ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
// 		ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
// 		T,
// 	> where
// 	DRAIN: 'static + Clone + Default + Debug + Stream<EP=DRAIN,T=T>,
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone,
// 	LU: Clone,
// 	PCNC: Clone + Debug,
// 	PCNU: Clone + Debug,
// 	PR: Clone + Debug + Transform<T = T>,
// 	PV: Clone,
// 	T: 'static + AbsDiffEq<Epsilon=T> + CoordFloat + FloatConst,
// {
// 	type T = T;

// 	fn center(mut self, center: &Coordinate<T>) -> Self {
// 		self.base = self.base.center(center);
// 		self.reclip_adjust()
// 	}
// }
