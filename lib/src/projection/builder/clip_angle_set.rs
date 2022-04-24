use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::gen_clip_circle;
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
	PR: Clone + Transform<T = T>,
	RC: Clone + Stream<EP = DRAIN, T = T>,
	RU: Clone + Connectable<Output = RC, SC = PCNC> + Debug,
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
	type T = T;

	// Given an angle in degrees. Sets the internal clip angle and returns a builder
	// which uses the clip circle stratergy.
	fn clip_angle(self, angle: T) -> Self::Output {
		if angle == T::zero() {
			panic!("must call clip_angle_reset() instead");
		}

		let theta = angle.to_radians();
		let clip = gen_clip_circle::<DRAIN, PCNC, PCNU, PR, RC, RU, T>(theta);
		// Copy, Mutate - updating only theta and preclip_factory.
		let out = Self::Output {
			p_pcnc: PhantomData::<PCNC>,
			projection_raw: self.projection_raw,
			clip,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			x: self.x,
			y: self.y,

			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,

			delta2: self.delta2,
			lambda: self.lambda,
			phi: self.phi,

			alpha: self.alpha,
			k: self.k,

			theta: Some(theta),

			sx: self.sx,
			sy: self.sy,

			rotate: self.rotate,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			postclip: self.postclip,

			resample: self.resample,
			rotator: self.rotator,
		};

		// out.reset()
		out
	}
}
