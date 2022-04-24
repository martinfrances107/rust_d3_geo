use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::PointVisible;
use crate::identity::Identity;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::Builder as ProjectionBuilder;
use crate::projection::builder_mercator::NoClipU;
use crate::projection::ClipExtentBounded;
use crate::projection::TransformExtent;
use crate::stream::Stream;
use crate::Coordinate;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentBounded
	for Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,
	PR: TransformExtent<T>,
	PV: PointVisible<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// f64 or f32.
	type T = T;
	type OutputClear =
		Builder<DRAIN, I, LB, LC, LU, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, PR, PV, RC, RU, T>;

	/// Returns a bounding box.
	#[inline]
	fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]> {
		self.extent
	}

	/// Clears the bounding box.
	fn clip_extent_clear(self) -> Self::OutputClear {
		let base = self.base;

		let base_out = ProjectionBuilder {
			p_pcnc: PhantomData::<NoClipC<DRAIN, T>>,
			projection_raw: base.projection_raw,
			clip: base.clip,
			phi: base.phi,
			lambda: base.lambda,
			alpha: base.alpha,
			k: base.k,
			sx: base.sx,
			sy: base.sy,
			x: base.x,
			y: base.y,
			delta_lambda: base.delta_lambda,
			delta_phi: base.delta_phi,
			delta_gamma: base.delta_gamma,
			delta2: base.delta2,
			theta: base.theta,
			rotate: base.rotate,
			project_transform: base.project_transform,
			project_rotate_transform: base.project_rotate_transform,
			resample: base.resample,
			rotator: base.rotator,

			// Mutate stage
			x0: None,
			y0: None,
			x1: None,
			y1: None,
			postclip: Identity::default(),
		};

		let out = Builder {
			pr: self.pr,
			base: base_out,
			extent: None,
		};
		// out.reset()
		out
	}
}
