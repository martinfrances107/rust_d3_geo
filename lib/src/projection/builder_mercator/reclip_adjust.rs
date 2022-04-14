use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ClipExtentAdjust;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
use crate::rot::rotate_radians;
use crate::stream::Connectable;
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
		Resample<
			DRAIN,
			PR,
			ClipC<DRAIN, T>,
			ClipU<DRAIN, T>,
			ConnectedResample<ClipC<DRAIN, T>, T>,
			T,
		>,
		Resample<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Unconnected, T>,
		T,
	> where
	DRAIN: 'static + Clone + Default + Stream<EP = DRAIN, T = T>,
	I: Clone,
	LB: Clone,
	LC: Clone,
	LU: Clone,
	PR: Clone + Transform<T = T>,
	PV: Clone,

	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	fn reclip_adjust(self) -> Self {
		let k = T::PI() * self.get_scale();

		let rotate_raw = self.base.get_rotate();
		let t = rotate_radians(rotate_raw).invert(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
		let t = self.base.build().transform(&t);
		let ce = match (self.x0, self.y0, self.x1, self.y1) {
			(Some(x0), Some(y0), Some(x1), Some(y1)) => {
				// MercatorRaw and MercatorTransverseRaw supply different
				// transforms
				// todo!("must change transform based on PR");
				// but for now assume projectionMercator is being used.
				[
					Coordinate {
						x: (t.x - k).max(x0),
						y: y0,
					},
					Coordinate {
						x: (t.x + k).min(x1),
						y: y1,
					},
				]
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

		self.clip_extent_adjust(&ce)
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
	PR: Clone + Transform<T = T>,
	PV: Clone,

	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	fn reclip_adjust(self) -> Self {
		let k = T::PI() * self.get_scale();

		let rotate_raw = self.base.get_rotate();
		let t = rotate_radians(rotate_raw).invert(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
		let t = self.base.build().transform(&t);
		let ce = match (self.x0, self.y0, self.x1, self.y1) {
			(Some(x0), Some(y0), Some(x1), Some(y1)) => {
				// MercatorRaw and MercatorTransverseRaw supply different
				// transforms
				// todo!("must change transform based on PR");
				// but for now assume projectionMercator is being used.
				[
					Coordinate {
						x: (t.x - k).max(x0),
						y: y0,
					},
					Coordinate {
						x: (t.x + k).min(x1),
						y: y1,
					},
				]
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

		self.clip_extent_adjust(&ce)
	}
}
