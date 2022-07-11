use std::marker::PhantomData;

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
use crate::projection::builder::Clip;
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
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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

	/// Set the projection builder precision
	///
	/// delta is related to clip angle.
	fn precision(self, delta: &T) -> Self::Output {
		let pv = PVAntimeridian::default();
		let interpolator = InterpolateAntimeridian::default();
		let line = LineAntimeridian::default();
		let delta2 = *delta * *delta;
		let resample = Resample::new(self.project_transform.clone(), delta2);
		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let clip = Clip::new(interpolator, line, pv, self.clip.start);

		// Copy - Mutate.
		let out = Self::Output {
			p_drain: PhantomData::<DRAIN>,
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
			delta2,
			resample,
		};

		// out.reset()
		out
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
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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

	/// Set the projection builder precision
	///
	/// delta is related to clip angle.
	fn precision(self, delta: &T) -> Self::Output {
		let radius = self.clip.interpolator.radius;
		let pv = PVCircle::new(radius);
		let interpolator = InterpolateCircle::new(radius);
		let line = LineCircle::default();
		let delta2 = *delta * *delta;
		let resample = Resample::new(self.project_transform.clone(), delta2);
		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let clip = Clip::new(interpolator, line, pv, self.clip.start);

		// Copy - Mutate.
		let out = Self::Output {
			p_drain: PhantomData::<DRAIN>,
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
			delta2,
			resample,
		};

		// out.reset()
		out
	}
}
