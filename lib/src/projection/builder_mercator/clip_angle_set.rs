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
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder::Builder;

impl<DRAIN, PCNC, PCNU, PR, RC, RU, T> ClipAngleSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<DRAIN, RC, T>,
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
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = Builder<
		DRAIN,
		InterpolateCircle<DRAIN, RC, T>,
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
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,
		}
	}
}
