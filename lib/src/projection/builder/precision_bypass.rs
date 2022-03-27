use crate::clip::buffer::Buffer;
use crate::projection::builder::Clip;
use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionBypass;
use crate::stream::Connected;
use crate::stream::Unconnected;
// use crate::Transform;

use super::Builder;

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionBypass
	for Builder<
		DRAIN,
		InterpolateAntimeridian<
			DRAIN,
			Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			T,
		>,
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
	> where
	PCNU: Clone + Debug,
	PCNC: Clone + Debug,
	DRAIN: Clone + Debug,
	PR: Clone + Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = Builder<
		DRAIN,
		InterpolateAntimeridian<DRAIN, None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>, T>,
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
	>;
	/// Set the projection builder precision
	///
	/// delta is related to clip angle.
	fn precision_bypass(self) -> Self::Output {
		let pv = PVAntimeridian::default();
		let interpolator = InterpolateAntimeridian::default();
		let line = LineAntimeridian::default();
		let resample = None::new(self.project_transform.clone());
		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let clip = Clip::new(interpolator, line, pv, self.clip.start);

		// Copy - Mutate.
		let out = Self::Output {
			p_lb: PhantomData::<LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>>,
			p_pcnc: PhantomData::<PCNC>,
			sx: self.sx,
			sy: self.sy,
			x: self.x,
			y: self.y,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,
			theta: self.theta,
			rotate: self.rotate,
			rotator: self.rotator,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			postclip: self.postclip,
			alpha: self.alpha,
			lambda: self.lambda,
			phi: self.phi,
			projection_raw: self.projection_raw,
			k: self.k,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			clip,

			// Mutate section.
			delta2: T::zero(),
			resample,
		};

		// out.reset()
		out
	}
}

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionBypass
	for Builder<
		DRAIN,
		InterpolateCircle<DRAIN, Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>, T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			Connected<Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>,
			T,
		>,
		LineCircle<
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
	> where
	PR: Clone,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = Builder<
		DRAIN,
		InterpolateCircle<DRAIN, None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>, T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
			Connected<None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>,
			T,
		>,
		LineCircle<DRAIN, None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVCircle<T>,
		None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
		None<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	>;
	/// Set the projection builder precision
	///
	/// delta is related to clip angle.
	fn precision_bypass(self) -> Self::Output {
		// This is a bodge radius info in embedded in the start variable.
		let pv = PVCircle::new(-self.clip.start.y);
		let interpolator = InterpolateCircle::new(-self.clip.start.y);
		let line = LineCircle::default();
		let resample = None::new(self.project_transform.clone());
		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let clip = Clip::new(interpolator, line, pv, self.clip.start);

		// Copy - Mutate.
		let out = Self::Output {
			p_lb: PhantomData::<LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>>,
			p_pcnc: PhantomData::<PCNC>,
			sx: self.sx,
			sy: self.sy,
			x: self.x,
			y: self.y,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,
			theta: self.theta,
			rotate: self.rotate,
			rotator: self.rotator,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			postclip: self.postclip,
			// rotate_transform: self.rotate_transform,
			alpha: self.alpha,
			lambda: self.lambda,
			phi: self.phi,
			projection_raw: self.projection_raw,
			k: self.k,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			clip,

			// Mutate section.
			delta2: T::zero(),
			resample,
		};

		// out.reset()
		out
	}
}
