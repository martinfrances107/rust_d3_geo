use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::resampler::none::None;

use crate::projection::ClipExtentAdjust;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
use crate::projection::TransformExtent;
use crate::rot::rotate_radians;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
use super::ReclipAdjust;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReclipAdjust
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PV,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: 'static + Clone + Default + Stream<EP = DRAIN, T = T>,
	I: Clone,
	LB: Clone,
	LC: Clone,
	LU: Clone,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	PV: Clone,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	fn reclip_adjust(mut self) -> Self {
		let k = T::PI() * self.get_scale();

		let rotate_raw = self.base.get_rotate();
		let t = rotate_radians(rotate_raw).invert(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
		let t = self.base.build().transform(&t);
		let ce = match self.extent {
			Some(extent) => {
				// MercatorRaw and MercatorTransverseRaw supply different
				// transforms
				// todo!("must change transform based on PR");
				// but for now assume projectionMercator is being used.
				self.pr.clone().transform_extent(
					k,
					t,
					extent[0].x,
					extent[0].y,
					extent[1].x,
					extent[1].y,
				)
			}
			_ => [
				Coordinate {
					x: t.x - k,
					y: t.y - k,
				},
				Coordinate {
					x: t.x + k,
					y: t.y + k,
				},
			],
		};

		self.base = self.base.clip_extent_adjust(&ce);
		self
	}
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReclipAdjust
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PV,
		None<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Connected<ClipC<DRAIN, T>>, T>,
		None<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Unconnected, T>,
		T,
	> where
	DRAIN: 'static + Clone + Default + Stream<EP = DRAIN, T = T>,
	I: Clone,
	LB: Clone,
	LC: Clone,
	LU: Clone,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	PV: Clone,

	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	fn reclip_adjust(mut self) -> Self {
		let k = T::PI() * self.get_scale();

		let rotate_raw = self.base.get_rotate();
		let t = rotate_radians(rotate_raw).invert(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
		let t = self.base.build().transform(&t);
		let ce = match self.extent {
			Some(extent) => {
				// MercatorRaw and MercatorTransverseRaw supply different
				// transforms
				// todo!("must change transform based on PR");
				// but for now assume projectionMercator is being used.
				self.pr.clone().transform_extent(
					k,
					t,
					extent[0].x,
					extent[0].y,
					extent[1].x,
					extent[1].y,
				)
			}
			_ => [
				Coordinate {
					x: t.x - k,
					y: t.y - k,
				},
				Coordinate {
					x: t.x + k,
					y: t.y + k,
				},
			],
		};
		self.base = self.base.clip_extent_adjust(&ce);
		self
	}
}
