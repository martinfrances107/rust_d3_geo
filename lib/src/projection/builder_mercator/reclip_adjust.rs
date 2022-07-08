use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoneClip;
use crate::projection::Build;
use crate::projection::ClipExtentAdjust;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
use crate::projection::TransformExtent;
use crate::rot::rotate_radians;
use crate::stream::Stream;

use crate::Transform;

use super::ReclipAdjust;

impl<DRAIN, PR, T> ReclipAdjust for BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
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

impl<DRAIN, PR, T> ReclipAdjust for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
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
