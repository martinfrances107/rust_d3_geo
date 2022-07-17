use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;

use crate::identity::Identity;
use crate::projection::ClipExtentBounded;

use super::template::ClipU;
use super::template::NoClipU;
use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentBounded
	for Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
	T: CoordFloat,
{
	type T = T;
	type OutputClear = Builder<DRAIN, I, LB, LC, LU, NoClipU<DRAIN>, PR, PV, RC, RU, T>;

	fn clip_extent_clear(self) -> Self::OutputClear {
		let out = Self::OutputClear {
			p_lb: PhantomData::<LB>,
			p_drain: PhantomData::<DRAIN>,
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
			x0: None,
			y0: None,
			x1: None,
			y1: None,
			postclip: Identity::default(),
		};
		//TODO must restore reset
		// out.reset()
		out
	}
}
