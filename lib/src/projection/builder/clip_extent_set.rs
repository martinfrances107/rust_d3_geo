use crate::stream::Stream;
use core::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::circle::gen_clip_circle;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoneClip;
use crate::projection::builder::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::BuilderAntimeridianResampleNoneNoClip;
use crate::projection::builder::BuilderCircleResampleNoClip;
use crate::projection::builder::BuilderCircleResampleNoneNoClip;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipExtentSet;
use crate::Transform;

use super::template::ClipC;
use super::template::ClipU;

// Code Repeated 2^2 times.
// Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Varariantion over Resample/None as Resample is rebuilt.
impl<DRAIN, PR, T> ClipExtentSet for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type OutputBounded = BuilderAntimeridianResampleClip<DRAIN, PR, T>;
	type T = T;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let clip = gen_clip_antimeridian::<
			DRAIN,
			ClipC<DRAIN, T>,
			ClipU<DRAIN, T>,
			PR,
			ResampleClipC<DRAIN, PR, T>,
			ResampleClipU<DRAIN, PR, T>,
			T,
		>();
		let resample = Resample::new(self.project_transform.clone(), self.delta2);
		let out = Self::OutputBounded {
			p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
			projection_raw: self.projection_raw,
			clip,
			phi: self.phi,
			lambda: self.lambda,
			alpha: self.alpha,
			k: self.k,
			sx: self.sx,
			sy: self.sy,
			x: self.x,
			y: self.y,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			delta2: self.delta2,
			theta: self.theta,
			rotate: self.rotate,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			resample,
			rotator: self.rotator,

			// Mutate stage
			x0: Some(extent[0].x),
			y0: Some(extent[0].y),
			x1: Some(extent[1].x),
			y1: Some(extent[1].y),
			postclip: ClipU::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
		};

		// out.reset()
		out
	}
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>
where
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type OutputBounded = BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>;
	type T = T;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let clip = gen_clip_antimeridian::<
			DRAIN,
			ClipC<DRAIN, T>,
			ClipU<DRAIN, T>,
			PR,
			ResampleNoneClipC<DRAIN, PR, T>,
			ResampleNoneClipU<DRAIN, PR, T>,
			T,
		>();
		let resample = None::new(self.project_transform.clone());
		let out = Self::OutputBounded {
			p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
			projection_raw: self.projection_raw,
			clip,
			phi: self.phi,
			lambda: self.lambda,
			alpha: self.alpha,
			k: self.k,
			sx: self.sx,
			sy: self.sy,
			x: self.x,
			y: self.y,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			delta2: self.delta2,
			theta: self.theta,
			rotate: self.rotate,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			resample,
			rotator: self.rotator,

			// Mutate stage
			x0: Some(extent[0].x),
			y0: Some(extent[0].y),
			x1: Some(extent[1].x),
			y1: Some(extent[1].y),
			postclip: ClipU::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
		};

		// out.reset()
		out
	}
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderCircleResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type OutputBounded = BuilderCircleResampleClip<DRAIN, PR, T>;
	type T = T;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let clip = gen_clip_circle::<
			DRAIN,
			ClipC<DRAIN, T>,
			ClipU<DRAIN, T>,
			PR,
			ResampleClipC<DRAIN, PR, T>,
			ResampleClipU<DRAIN, PR, T>,
			T,
		>(self.theta.unwrap());
		let resample = Resample::new(self.project_transform.clone(), self.delta2);
		let out = Self::OutputBounded {
			p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
			projection_raw: self.projection_raw,
			clip,
			phi: self.phi,
			lambda: self.lambda,
			alpha: self.alpha,
			k: self.k,
			sx: self.sx,
			sy: self.sy,
			x: self.x,
			y: self.y,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			delta2: self.delta2,
			theta: self.theta,
			rotate: self.rotate,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			resample,
			rotator: self.rotator,

			// Mutate stage
			x0: Some(extent[0].x),
			y0: Some(extent[0].y),
			x1: Some(extent[1].x),
			y1: Some(extent[1].y),
			postclip: ClipU::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
		};

		// out.reset()
		out
	}
}

impl<DRAIN, PR, T> ClipExtentSet for BuilderCircleResampleNoneNoClip<DRAIN, PR, T>
where
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type OutputBounded = BuilderCircleResampleNoneClip<DRAIN, PR, T>;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let clip = gen_clip_circle::<
			DRAIN,
			ClipC<DRAIN, T>,
			ClipU<DRAIN, T>,
			PR,
			ResampleNoneClipC<DRAIN, PR, T>,
			ResampleNoneClipU<DRAIN, PR, T>,
			T,
		>(self.theta.unwrap());
		let resample = None::new(self.project_transform.clone());
		let out = Self::OutputBounded {
			p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
			projection_raw: self.projection_raw,
			clip,
			phi: self.phi,
			lambda: self.lambda,
			alpha: self.alpha,
			k: self.k,
			sx: self.sx,
			sy: self.sy,
			x: self.x,
			y: self.y,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			delta2: self.delta2,
			theta: self.theta,
			rotate: self.rotate,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			resample,
			rotator: self.rotator,

			// Mutate stage
			x0: Some(extent[0].x),
			y0: Some(extent[0].y),
			x1: Some(extent[1].x),
			y1: Some(extent[1].y),
			postclip: ClipU::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y),
		};

		// out.reset()
		out
	}
}
