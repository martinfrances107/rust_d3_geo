use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

//TODO before release add more variants here
// vary by LineClip,

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
			Connected<None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>,
			T,
		>,
		LineAntimeridian<None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVAntimeridian<T>,
		None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
		None<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	> where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,

	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Output = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			Connected<Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>,
			T,
		>,
		LineAntimeridian<Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVAntimeridian<T>,
		Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
		Resample<PR, PCNC, PCNU, Unconnected, T>,
		T,
	>;
	type T = T;

	fn precision(self, delta: &T) -> Self::Output {
		let base = self.base.precision(delta);
		Self::Output {
			extent: self.extent,
			pr: self.pr,
			base,
		}
	}
}

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
	for Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
			Connected<None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>,
			T,
		>,
		LineCircle<None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVCircle<T>,
		None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
		None<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	> where
	DRAIN: Default + Debug + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Output = Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			Connected<Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>,
			T,
		>,
		LineCircle<Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVCircle<T>,
		Resample<PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
		Resample<PR, PCNC, PCNU, Unconnected, T>,
		T,
	>;
	type T = T;

	fn precision(self, delta: &T) -> Self::Output {
		let base = self.base.precision(delta);
		Self::Output {
			extent: self.extent,
			pr: self.pr,
			base,
		}
	}
}
