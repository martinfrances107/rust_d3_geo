use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::ClipAngleSet;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, PCNC, PCNU, PR, RC, RU, T> ClipAngleSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<DRAIN, RC, Connected<RC>, T>,
		LineAntimeridian<DRAIN, RC, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVAntimeridian<T>,
		RC,
		RU,
		T,
	> where
	PCNU: Clone + Connectable<Output = PCNC, SC = DRAIN>,
	RU: Debug,
	RC: Clone + Stream<EP = DRAIN, T = T>,
	RU: Clone + Connectable<Output = RC, SC = PCNC> + Debug,
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<DRAIN, RC, Connected<RC>, T>,
		LineCircle<DRAIN, RC, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVCircle<T>,
		RC,
		RU,
		T,
	>;
	/// f32 or f64.
	type T = T;

	// Given an angle in degrees. Sets the internal clip angle and returns a builder
	// which uses the clip circle stratergy.
	fn clip_angle(self, angle: T) -> Self::Output {
		let base = self.base.clip_angle(angle);
		Self::Output {
			pr: self.pr,
			base,
			extent: self.extent,
		}
	}
}
