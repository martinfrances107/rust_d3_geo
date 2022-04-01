use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::circle::gen_clip_circle;
use crate::clip::circle::interpolate::Interpolate;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;
use crate::projection::ClipAngleAdjust;
use crate::stream::Connected;
use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, PCNC, PCNU, PR, RC, RU, T> ClipAngleAdjust
	for Builder<
		DRAIN,
		Interpolate<T>,
		Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		Line<DRAIN, RC, Connected<RC>, T>,
		Line<DRAIN, RC, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PV<T>,
		RC,
		RU,
		T,
	> where
	PCNU: Debug,
	RU: Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn clip_angle_adjust(mut self, angle: T) -> Self {
		if angle == T::zero() {
			panic!("must call clip_angle_reset() instead");
		}
		let theta = angle.to_radians();
		let clip = gen_clip_circle::<DRAIN, PCNC, PCNU, PR, RC, RU, T>(theta);

		self.clip = clip;
		self.theta = Some(angle);

		// TODO must reinstate.
		// self.reset()
		self
	}
}
