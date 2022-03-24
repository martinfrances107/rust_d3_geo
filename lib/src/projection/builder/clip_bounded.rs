// use crate::projection::builder::Buffer;
use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

// use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
// use crate::clip::antimeridian::line::Line as LineAntimeridian;
// use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::projection::ClipExtentBounded;
use crate::Transform;

// use crate::clip::rectangle::Rectangle;
// use crate::clip::Interpolate;
// use crate::clip::PointVisible;
use crate::identity::Identity;
// use crate::projection::resampler::Resampler;
// use crate::stream::Connectable;
// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Unconnected;

use super::Builder;
// use super::ClipExtentBounded;

use super::template::ClipC;
use super::template::ClipU;
use super::template::NoClipC;
use super::template::NoClipU;
// use super::ProjectionRawBase;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentBounded
	for Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
	DRAIN: Clone + Debug,
	I: Clone,
	LB: Clone,
	LC: Clone + Debug,
	LU: Clone + Debug,
	PR: Transform<T = T>,
	PV: Clone + Debug,
	// I: Interpolate<T = T>,
	// DRAIN: Stream<EP = DRAIN, T = T> + Default,
	// PR: ProjectionRawBase<T>,
	// PV: PointVisible<T = T>,
	// RC: Resampler + Stream<EP = DRAIN, T = T>,
	// RU: Resampler + Connectable<Output = RC, SC = Rectangle<DRAIN, DRAIN, Connected<DRAIN>, T>>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;
	type OutputClear =
		Builder<DRAIN, I, LB, LC, LU, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, PR, PV, RC, RU, T>;

	fn get_clip_extent(&self) -> Option<[Coordinate<T>; 2]> {
		match (self.x0, self.y0, self.x1, self.y1) {
			(Some(x0), Some(y0), Some(x1), Some(y1)) => {
				Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
			}
			_ => None,
		}
	}

	fn clip_extent_clear(self) -> Self::OutputClear {
		// let p_pcnc = PhantomData::<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>>;
		let out = Self::OutputClear {
			// p_pcnc,
			// p_lb: PhantomData::<LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>>,
			p_lb: PhantomData::<LB>,
			p_pcnc: PhantomData::<NoClipC<DRAIN, T>>,
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
			// rotate_transform: self.rotate_transform,
			rotator: self.rotator,

			// Mutate stage
			x0: None,
			y0: None,
			x1: None,
			y1: None,
			postclip: Identity::default(),
		};
		// out.reset()
		out
	}
}
