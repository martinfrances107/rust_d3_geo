use core::marker::PhantomData;
use std::fmt::Debug;

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
use crate::clip::clip::Connected as ConnectedClip;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::ClipExtentSet;
use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentSet
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		// InterpolateAntimeridian<T>,
		// LineAntimeridian<Buffer<T>, Buffer<T>, STATE_B, T>,
		// LineAntimeridian<DRAIN, RC, STATE_C, T>,
		// LineAntimeridian<DRAIN, RC, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PV,
		RC,
		RU,
		T,
	> where
	RC: Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded = Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		// InterpolateAntimeridian<T>,
		// LineAntimeridian<Buffer<T>, Buffer<T>, STATE_B, T>,
		// LineAntimeridian<DRAIN, RC, STATE_C, T>,
		// LineAntimeridian<DRAIN, RC, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PV,
		RC,
		RU,
		T,
	>;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let base = self.base.clip_extent(extent);

		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let out = Self::OutputBounded {
			// p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
			base,
			pr: self.pr,
			// Mutate stage
			x0: Some(extent[0].x),
			y0: Some(extent[0].y),
			x1: Some(extent[1].x),
			y1: Some(extent[1].y),
		};
		// .reset();

		// out.reset()
		out
	}
}

// impl<DRAIN, PR, PV, RC, RU, STATE_B, STATE_C, T> ClipExtentSet
// 	for Builder<
// 		DRAIN,
// 		InterpolateCircle<T>,
// 		LineCircle<Buffer<T>, Buffer<T>, STATE_B, T>,
// 		LineCircle<DRAIN, RC, STATE_C, T>,
// 		LineCircle<DRAIN, RC, Unconnected, T>,
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
// 		InterpolateCircle<T>,
// 		LineCircle<Buffer<T>, Buffer<T>, STATE_B, T>,
// 		LineCircle<DRAIN, RC, STATE_C, T>,
// 		LineCircle<DRAIN, RC, Unconnected, T>,
// 		ClipC<DRAIN, T>,
// 		ClipU<DRAIN, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>;

// 	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
// 		let base = self.base.clip_extent(extent);

// 		// Architecture Discussion:
// 		// CLIP is generic over <.. RC, RU,..>,
// 		// So a change in the resample type causes rebuilding of clip.
// 		let out = Self::OutputBounded {
// 			// p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
// 			base,
// 			pr: self.pr,
// 			// Mutate stage
// 			x0: Some(extent[0].x),
// 			y0: Some(extent[0].y),
// 			x1: Some(extent[1].x),
// 			y1: Some(extent[1].y),
// 		};
// 		// .reset();

// 		// out.reset()
// 		out
// 	}
// }
