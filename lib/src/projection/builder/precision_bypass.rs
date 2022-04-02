use std::fmt::Debug;
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
use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder::Clip;
use crate::projection::builder::NoClipC;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionBypass;
use crate::stream::Connected;
use crate::stream::Unconnected;

// Vary by Clipping stratergy ( Antimeridna/Circle).
// Vary by Post Clip Node (Identity/Rectangle).
// So 2^2 variations .. repeats of similar code.

use super::Builder;

impl<DRAIN, PR, T> PrecisionBypass
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Clone + Debug,
	PR: Clone + Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoneNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
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
			// p_pcnc: PhantomData::<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>>,
			p_pcnc: PhantomData::<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>>,
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

impl<DRAIN, PR, T> PrecisionBypass
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			Resample<
				DRAIN,
				PR,
				Rectangle<DRAIN, DRAIN, Connected<DRAIN>, T>,
				Rectangle<DRAIN, DRAIN, Unconnected, T>,
				ConnectedResample<Rectangle<DRAIN, DRAIN, Connected<DRAIN>, T>, T>,
				T,
			>,
			Connected<ResampleClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Clone + Debug,
	PR: Clone + Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoneClipC<DRAIN, PR, T>,
			Connected<ResampleNoneClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
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
			p_pcnc: PhantomData::<Rectangle<DRAIN, DRAIN, Connected<DRAIN>, T>>,
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

impl<DRAIN, PR, T> PrecisionBypass
	for Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Debug,
	PR: Clone + Debug,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoneNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
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
			// p_lb: PhantomData::<LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>>,
			p_pcnc: PhantomData::<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>>,
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

impl<DRAIN, PR, T> PrecisionBypass
	for Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<DRAIN, ResampleClipC<DRAIN, PR, T>, Connected<ResampleClipC<DRAIN, PR, T>>, T>,
		LineCircle<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Debug,
	PR: Clone + Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoneClipC<DRAIN, PR, T>,
			Connected<ResampleNoneClipC<DRAIN, PR, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
		Rectangle<DRAIN, DRAIN, Connected<DRAIN>, T>,
		Rectangle<DRAIN, DRAIN, Unconnected, T>,
		PR,
		PVCircle<T>,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
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
			p_pcnc: PhantomData::<Rectangle<DRAIN, DRAIN, Connected<DRAIN>, T>>,
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
