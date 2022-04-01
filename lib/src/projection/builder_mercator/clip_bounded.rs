use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::PointVisible;
use crate::identity::Identity;
use crate::projection::builder::Builder as ProjectionBuilder;
use crate::projection::ClipExtentBounded;
use crate::projection::TransformExtent;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Coordinate;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> ClipExtentBounded
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	DRAIN: Default + Debug + Stream<EP = DRAIN, T = T>,
	RU: Debug,
	PCNU: Debug,
	PR: TransformExtent<T>,
	PV: PointVisible<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// f64 or f32.
	type T = T;
	type OutputClear = Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
		Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
		PR,
		PV,
		RC,
		RU,
		T,
	>;

	// Returns a bounding box.
	fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]> {
		match (self.x0, self.y0, self.x1, self.y1) {
			(Some(x0), Some(y0), Some(x1), Some(y1)) => {
				Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
			}
			_ => None,
		}
	}

	// clears the bounding box.
	fn clip_extent_clear(self) -> Self::OutputClear {
		let base = self.base;

		let base_out = ProjectionBuilder {
			p_pcnc: PhantomData::<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>>,
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
			x0: None,
			y0: None,
			x1: None,
			y1: None, //
		};
		// out.reset()
		out
	}

	// Sets the bounding box.
	// fn clip_extent(mut self, extent: &[Coordinate<Self::T>; 2]) -> Self::OutputBounded {
	// 	self.x0 = Some(extent[0].x);
	// 	self.y0 = Some(extent[0].y);
	// 	self.x1 = Some(extent[1].x);
	// 	self.y1 = Some(extent[1].y);
	// 	self.reclip()
	// }
}
