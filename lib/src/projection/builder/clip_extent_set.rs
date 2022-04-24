use core::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimerdian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::gen_clip_circle;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipExtentSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::template::ClipC;
use super::template::ClipU;
use super::template::NoClipC;
use super::template::NoClipU;
use super::Builder;

// Code Repeated 2^2 times.
// Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Varariantion over Resample/None as Resample is rebuilt.
impl<DRAIN, PR, T> ClipExtentSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimerdian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimerdian<
			DRAIN,
			ResampleNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimerdian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: 'static + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimerdian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimerdian<
			DRAIN,
			ResampleClipC<DRAIN, PR, T>,
			Connected<ResampleClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimerdian<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	>;

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

impl<DRAIN, PR, T> ClipExtentSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimerdian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimerdian<
			DRAIN,
			ResampleNoneNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimerdian<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: 'static + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type OutputBounded = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimerdian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimerdian<
			DRAIN,
			ResampleNoneClipC<DRAIN, PR, T>,
			Connected<ResampleNoneClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimerdian<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	>;
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

impl<DRAIN, PR, T> ClipExtentSet
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
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type OutputBounded = Builder<
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
	>;
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

impl<DRAIN, PR, T> ClipExtentSet
	for Builder<
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
	> where
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded = Builder<
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
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	>;

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
