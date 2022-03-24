use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::projection::builder::Clip;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
	for Builder<
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
	> where
	PCNU: Clone + Debug,
	PCNC: Clone + Debug,
	DRAIN: Clone + Debug,
	PR: Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type Output = Builder<
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
	>;
	/// Set the projection builder precision
	///
	/// delta is related to clip angle.
	fn precision(self, delta: &T) -> Self::Output {
		let pv = PVAntimeridian::default();
		let interpolator: InterpolateAntimeridian<
			DRAIN,
			Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			T,
		> = InterpolateAntimeridian::default();
		let line = LineAntimeridian::default();
		let delta2 = *delta * *delta;
		let resample: Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T> =
			Resample::new(self.project_transform.clone(), delta2);
		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let clip: Clip<
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
			PR,
			PVAntimeridian<T>,
			Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
			Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
			Unconnected,
			T,
		> = Clip::new(interpolator, line, pv, self.clip.start);

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
			delta2,
			resample,
		};

		out.reset()
	}
}

// TODO - Must do for LineCircle, etc.
// impl<DRAIN, Iin,  LB, LC, LU, PCNC, PCNU, PR, PV, T> PrecisionSet
// 	for Builder<
// 		DRAIN,
// 		Iin,
// 		LB,
// 		LC,
// 		LU,
// 		PCNC,
// 		PCNU,
// 		PR,
// 		PV,
// 		None<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
// 		None<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
// 		T,
// 	> where
// 	// I: Interpolate<T = T>,
// 	// DRAIN: Stream<EP = DRAIN, T = T> + Default,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
// 	// PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
// 	DRAIN: Clone + Debug,
// 	Iin: Clone,
// 	PCNU: Clone + Debug,
// 	PCNC: Clone + Debug,
// 	LB: Clone,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PV: Clone + Debug,
// 	PCNC: Clone,
// 	PR: Transform<T = T>,
// 	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// 	type T = T;
// 	type Output = Builder<
// 		DRAIN,
// 		_,
// 		LB,
// 		LC,
// 		LU,
// 		PCNC,
// 		PCNU,
// 		PR,
// 		PV,
// 		Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
// 		Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
// 		T,
// 	>;
// 	/// Set the projection builder precision
// 	///
// 	/// delta is related to clip angle.
// 	fn precision(self, delta: &T) -> Self::Output {
// 		let delta2 = *delta * *delta;
// 		let resample = Resample::new(self.project_transform.clone(), delta2);
// 		// Architecture Discussion:
// 		// CLIP is generic over <.. RC, RU,..>,
// 		// So a change in the resample type causes rebuilding of clip.
// 		// the implicit (_) type are auto derived from the 'clip_line'
// 		let clip: Clip<
// 			DRAIN,
// 			_,
// 			LB,
// 			_,
// 			_,
// 			PR,
// 			PV,
// 			Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
// 			Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
// 			Unconnected,
// 			T,
// 		> = Clip::new(
// 			self.clip.interpolator,
// 			self.clip.clip_line,
// 			self.clip.pv,
// 			self.clip.start,
// 		);
// 		// Copy - Mutate.
// 		let out = Self::Output {
// 			// p_pcnc: self.p_pcnc,
// 			p_lb: PhantomData::<LB>,
// 			// p_lc: PhantomData::<LC>,
// 			p_pcnc: PhantomData::<PCNC>,
// 			sx: self.sx,
// 			sy: self.sy,
// 			x: self.x,
// 			y: self.y,
// 			x0: self.x0,
// 			y0: self.y0,
// 			x1: self.x1,
// 			y1: self.y1,
// 			theta: self.theta,
// 			rotate: self.rotate,
// 			rotator: self.rotator,
// 			project_transform: self.project_transform,
// 			project_rotate_transform: self.project_rotate_transform,
// 			postclip: self.postclip,
// 			// rotate_transform: self.rotate_transform,
// 			alpha: self.alpha,
// 			lambda: self.lambda,
// 			phi: self.phi,
// 			projection_raw: self.projection_raw,
// 			k: self.k,
// 			delta_lambda: self.delta_lambda,
// 			delta_phi: self.delta_phi,
// 			delta_gamma: self.delta_gamma,
// 			clip,

// 			// Mutate section.
// 			delta2,
// 			resample,
// 		};

// 		out.reset()
// 	}
// }
