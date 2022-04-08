use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder_mercator::ResampleNoClipC;
use crate::projection::builder_mercator::ResampleNoClipU;
use crate::stream::Connected;
use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::clip::antimeridian::gen_clip_antimeridian;
// use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
// use crate::clip::antimeridian::line::Line as LineAntimeridian;
// use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
// use crate::clip::buffer::Buffer;
// use crate::clip::PointVisible;
// use crate::identity::Identity;
// use crate::projection::builder::template::ResampleNoClipC;
// use crate::projection::builder::template::ResampleNoClipU;
// use crate::projection::builder::Builder as ProjectionBuilder;

use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder_mercator::Builder as MercatorBuilder;
use crate::projection::ClipExtentSet;
use crate::rot::rotate_radians;
// use crate::projection::resampler::none::None as ResampleNone;
// use crate::projection::resampler::resample::Connected as ConnectedResample;
// use crate::projection::resampler::resample::Resample;
// use crate::projection::stream_transform_radians::StreamTransformRadians;
// use crate::projection::ClipExtentBounded;
// use crate::projection::Projector;
use crate::projection::TransformExtent;
// use crate::projection::TranslateGet;
use crate::clip::buffer::Buffer;
// use crate::projection::TranslateSet;
use crate::projection::RotateGet;
use crate::projection::ScaleGet;
// use crate::stream::Connected;
// use crate::stream::Stream;
use crate::Coordinate;
use crate::Transform;

use super::Builder;

use crate::stream::Unconnected;

impl<DRAIN, PR, T>
	Builder<
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
	DRAIN: Clone,
	PR: Clone + Transform<T = T>,

	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	fn reclip(
		mut self,
	) -> Builder<
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
	> {
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
				todo!("must change transform based on PR");
				// self.clip_extent(k, t, x0, y0, x1, y1)
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

		self.clip_extent(&ce)
	}
}

impl<DRAIN, PR, T>
	Builder<
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
	DRAIN: Clone,
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	fn reclip(
		mut self,
	) -> Builder<
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
	> {
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
				todo!("must change transform based on PR");
				// self.clip_extent(k, t, x0, y0, x1, y1)
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

		self.clip_extent(&ce)
	}
}
