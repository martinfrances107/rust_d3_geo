use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::ClipAngleReset;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, PCNC, PCNU, RC, RU, PR, T> ClipAngleReset
	for Builder<
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
	> where
	PCNU: Debug,
	RC: Stream<EP = DRAIN, T = T>,
	RU: Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = Builder<
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
	>;
	type T = T;
	// Set the internal clip angle (theta) to null and return a builder
	// which uses the antimeridian clipping stratergy.
	fn clip_angle_reset(self) -> Self::Output {
		let clip = gen_clip_antimeridian::<DRAIN, PCNC, PCNU, PR, RC, RU, T>();

		// update only theta and preclip_factory.
		let out = Self::Output {
			clip,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			p_pcnc: self.p_pcnc,
			rotator: self.rotator,
			// p_lb: PhantomData::<LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>>,
			projection_raw: self.projection_raw,
			postclip: self.postclip,
			x: self.x,
			y: self.y,
			resample: self.resample,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,
			delta2: self.delta2,
			lambda: self.lambda,
			phi: self.phi,

			alpha: self.alpha,
			k: self.k,
			theta: None,
			sx: self.sx,
			sy: self.sy,
			rotate: self.rotate.clone(),
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
		};

		// TODO must reinstate reset.
		// out.reset()
		out
	}
}
