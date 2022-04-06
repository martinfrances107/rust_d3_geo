use core::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::clip::rectangle::Rectangle;
use crate::projection::ClipExtentSet;
use crate::stream::Unconnected;

use super::template::ClipC;
use super::template::ClipU;
use super::template::NoClipC;
use super::template::NoClipU;
use super::Builder;

// impl<DRAIN, PR, PV, RC, RU, T> ClipExtentSet
// 	for Builder<
// 		DRAIN,
// 		InterpolateAntimeridian<T>,
// 		LineAntimeridian<Buffer<T>, Buffer<T>, Unconnected, T>,
// 		LineAntimeridian<DRAIN, RC, Unconnected, T>,
// 		LineAntimeridian<DRAIN, RC, Unconnected, T>,
// 		NoClipC<DRAIN, T>,
// 		NoClipU<DRAIN, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	> where
// 	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// 	type T = T;

// 	type OutputBounded = Builder<
// 		DRAIN,
// 		InterpolateAntimeridian<T>,
// 		LineAntimeridian<Buffer<T>, Buffer<T>, Unconnected, T>,
// 		LineAntimeridian<DRAIN, RC, Unconnected, T>,
// 		LineAntimeridian<DRAIN, RC, Unconnected, T>,
// 		ClipC<DRAIN, T>,
// 		ClipU<DRAIN, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>;

// 	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
// 		let out = Self::OutputBounded {
// 			p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
// 			projection_raw: self.projection_raw,
// 			clip: self.clip,
// 			phi: self.phi,
// 			lambda: self.lambda,
// 			alpha: self.alpha,
// 			k: self.k,
// 			sx: self.sx,
// 			sy: self.sy,
// 			x: self.x,
// 			y: self.y,
// 			delta_lambda: self.delta_lambda,
// 			delta_phi: self.delta_phi,
// 			delta_gamma: self.delta_gamma,
// 			delta2: self.delta2,
// 			theta: self.theta,
// 			rotate: self.rotate,
// 			project_transform: self.project_transform,
// 			project_rotate_transform: self.project_rotate_transform,
// 			resample: self.resample,
// 			rotator: self.rotator,

// 			// Mutate stage
// 			x0: Some(extent[0].x),
// 			y0: Some(extent[0].y),
// 			x1: Some(extent[1].x),
// 			y1: Some(extent[1].y),
// 			postclip: Rectangle::<DRAIN, DRAIN, Unconnected, T>::new(
// 				extent[0].x,
// 				extent[0].y,
// 				extent[1].x,
// 				extent[1].y,
// 			),
// 		};

// 		// out.reset()
// 		out
// 	}
// }

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentSet
	for Builder<DRAIN, I, LB, LC, LU, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded =
		Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let out = Self::OutputBounded {
			p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
			projection_raw: self.projection_raw,
			clip: self.clip,
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
			resample: self.resample,
			rotator: self.rotator,

			// Mutate stage
			x0: Some(extent[0].x),
			y0: Some(extent[0].y),
			x1: Some(extent[1].x),
			y1: Some(extent[1].y),
			postclip: Rectangle::<DRAIN, DRAIN, Unconnected, T>::new(
				extent[0].x,
				extent[0].y,
				extent[1].x,
				extent[1].y,
			),
		};

		// out.reset()
		out
	}
}
