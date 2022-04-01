use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
			Connected<None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVAntimeridian<T>,
		None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
		None<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	> where
	DRAIN: Default + Debug + Stream<EP = DRAIN, T = T>,
	PCNC: Debug,
	PCNU: Debug,
	PR: Clone + Debug + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type Output = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			Connected<Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>,
			T,
		>,
		LineAntimeridian<
			DRAIN,
			Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			Unconnected,
			T,
		>,
		PCNC,
		PCNU,
		PR,
		PVAntimeridian<T>,
		Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
		Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	>;
	type T = T;

	fn precision(self, delta: &T) -> Self::Output {
		let base = self.base.precision(delta);
		Self::Output {
			pr: self.pr,
			base,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,
		}
	}
}
