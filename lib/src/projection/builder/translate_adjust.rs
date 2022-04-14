use crate::clip::Bufferable;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::projection::builder::Buffer;
use crate::projection::builder::NoClipC;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Unconnected;
use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::TranslateAdjust;
use crate::Transform;

use super::template::ClipC;
use super::template::ClipU;
use super::Builder;

impl<DRAIN, I, LC, LB, LU, PR, PV, T> TranslateAdjust
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
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Debug,
	PR: Clone + Debug + Transform<T = T>,
	I: Interpolator<T = T>,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone + LineConnected<SC = ResampleNoClipC<DRAIN, PR, T>> + Stream<EP = DRAIN, T = T>,
	LU: Clone
		+ Connectable<Output = LC, SC = ResampleNoClipC<DRAIN, PR, T>>
		+ Bufferable<Output = LB, T = T>
		+ Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate_adjust(mut self, t: &Coordinate<T>) -> Self {
		self.x = t.x;
		self.y = t.y;
		self.recenter_with_resampling()
	}
}

impl<DRAIN, I, LC, LB, LU, PCNC, PCNU, PR, PV, T> TranslateAdjust
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
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Debug,
	PR: Clone + Debug + Transform<T = T>,
	I: Interpolator<T = T>,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone
		+ LineConnected<SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>
		+ Stream<EP = DRAIN, T = T>,
	LU: Clone
		+ Connectable<
			Output = LC,
			SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
		> + Bufferable<Output = LB, T = T>
		+ Debug,

	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate_adjust(mut self, t: &Coordinate<T>) -> Self {
		self.x = t.x;
		self.y = t.y;
		self.recenter_with_resampling()
	}
}

/// no resampling below.

impl<DRAIN, I, LC, LB, LU, PR, PV, T> TranslateAdjust
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
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Debug,
	PR: Clone + Debug + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate_adjust(mut self, t: &Coordinate<T>) -> Self {
		self.x = t.x;
		self.y = t.y;
		self.recenter_no_resampling()
	}
}
