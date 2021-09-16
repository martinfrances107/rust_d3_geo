use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::mercator::Mercator;
use crate::projection::Projection;
use crate::rotation::rotate_radians;
use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::generate as gen_interpolate;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder as ProjectionBuilder;
use super::stream_node_factory::StreamNodeFactory;
use super::stream_transform_radians::StreamTransformRadians;
use super::ClipExtent;
use super::Precision;
use super::Raw as ProjectionRaw;
use super::Scale;
use super::Translate;

/// A wrapper for Projectio\Builder which overrides the traits - scale translate and center.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct MercatorBuilder<DRAIN, L, PR, PV, T>
where
	DRAIN: Stream<T = T> + Default,
	L: Line,
	PV: PointVisible<T = T>,
	PR: ProjectionRaw<T>, // TODO limit this to only certain types of PR
	T: AsPrimitive<T> + CoordFloat + FloatConst,
{
	// pr: PR,
	base: ProjectionBuilder<DRAIN, L, PR, PV, T>,
	x0: Option<T>,
	y0: Option<T>,
	x1: Option<T>,
	y1: Option<T>, // post-clip extent
}

impl<DRAIN, PR, T> MercatorBuilder<DRAIN, LineAntimeridian<T>, PR, PVAntimeridian<T>, T>
where
	DRAIN: Stream<T = T> + Default,
	PR: ProjectionRaw<T>,
	T: 'static + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// Wrap a default projector and provides mercator specific overrides.
	pub fn new(pr: PR) -> Self {
		let base: ProjectionBuilder<DRAIN, LineAntimeridian<T>, PR, PVAntimeridian<T>, T> =
			ProjectionBuilder::new(
				StreamNodeClipFactory::new(
					gen_interpolate(),
					LineAntimeridian::<T>::default(),
					PVAntimeridian::default(),
				),
				pr,
			);
		// dbg!(&base.get_scale());
		Self {
			base,
			x0: None,
			y0: None,
			x1: None,
			y1: None,
		}
	}

	/// Using the currently programmed state output a new projection.
	#[inline]
	pub fn build(&self) -> Projection<DRAIN, LineAntimeridian<T>, PR, PVAntimeridian<T>, T> {
		Projection {
			postclip: self.base.postclip.clone(),
			preclip_factory: self.base.preclip_factory.clone(),
			resample_factory: self.base.resample_factory.clone(),

			rotate_transform: self.base.project_rotate_transform.clone(),
			rotate_transform_factory: self.base.rotate_transform_factory.clone(),
			rotate_factory: self.base.rotate_factory.clone(),
			transform_radians_factory: StreamNodeFactory::new(StreamTransformRadians {}),
		}
	}
}

impl<DRAIN, L, PR, PV, T> MercatorBuilder<DRAIN, L, PR, PV, T>
where
	DRAIN: Stream<T = T> + Default,
	L: Line,
	PR: ProjectionRaw<T>,
	PV: PointVisible<T = T>,
	T: 'static + AsPrimitive<T> + CoordFloat + FloatConst,
{
	fn reclip(mut self) -> Self {
		dbg!("mercator reclip");
		let k = T::PI() * self.get_scale();

		let rotate_raw = self.base.get_rotate();
		let t = rotate_radians(rotate_raw).invert(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
		println!("mercator reclip t first{:?}", t);
		let t = self.base.build().transform(&t);
		dbg!(&self.x0);
		dbg!(k);
		dbg!("t final", t);
		let ce = match (self.x0, self.y0, self.x1, self.y1) {
			(Some(_x0), Some(y0), Some(x1), Some(y1)) => {
				todo!("not yet");
				[
					Coordinate {
						x: (t.x - k).max(t.y - k),
						y: y0,
					},
					Coordinate {
						x: (t.x + k).min(x1),
						y: y1,
					},
				]
				// _ => [
				// 	Coordinate {
				// 		x: x0,
				// 		y: (t.y - k).max(self.y0),
				// 	},
				// 	Coordinate {
				// 		x: self.x1,
				// 		y: (t.y + k).min(self.y1),
				// 	},
				// ],
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

		dbg!(&ce);
		self.base = self.base.clip_extent(Some(ce));
		self
	}
}

impl<DRAIN, L, PR, PV, T> Scale for MercatorBuilder<DRAIN, L, PR, PV, T>
where
	DRAIN: Stream<T = T> + Default,
	L: Line,
	PR: ProjectionRaw<T>,
	PV: PointVisible<T = T>,
	T: 'static + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn get_scale(&self) -> T {
		self.base.get_scale()
	}

	fn scale(mut self, scale: T) -> Self {
		self.base = self.base.scale(scale);
		self.reclip()
	}
}

impl<DRAIN, L, PR, PV, T> Translate for MercatorBuilder<DRAIN, L, PR, PV, T>
where
	DRAIN: Stream<T = T> + Default,
	L: Line,
	PR: ProjectionRaw<T>,
	PV: PointVisible<T = T>,
	T: 'static + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn get_translate(&self) -> Coordinate<T> {
		self.base.get_translate()
	}

	fn translate(mut self, t: &Coordinate<T>) -> Self {
		self.base = self.base.translate(t);
		self.reclip()
	}
}

impl<DRAIN, L, PR, PV, T> Precision for MercatorBuilder<DRAIN, L, PR, PV, T>
where
	DRAIN: Stream<T = T> + Default,
	L: Line,
	PR: ProjectionRaw<T>,
	PV: PointVisible<T = T>,
	T: 'static + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;
	#[inline]
	fn get_precision(&self) -> T {
		self.base.get_precision()
	}

	fn precision(mut self, delta: &T) -> Self {
		self.base = self.base.precision(delta);
		self
	}
}

impl<DRAIN, L, PR, PV, T> ClipExtent for MercatorBuilder<DRAIN, L, PR, PV, T>
where
	DRAIN: Stream<T = T> + Default,
	L: Line,
	PR: ProjectionRaw<T>,
	PV: PointVisible<T = T>,
	T: 'static + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// f64 or f32
	type T = T;
	/// Returns a bounding box.
	fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]> {
		match (self.x0, self.y0, self.x1, self.y1) {
			(Some(x0), Some(y0), Some(x1), Some(y1)) => {
				Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
			}
			_ => None,
		}
	}

	/// Sets the bounding box.
	fn clip_extent(mut self, extent: Option<[Coordinate<Self::T>; 2]>) -> Self {
		match extent {
			Some(e) => {
				self.x0 = Some(e[0].x);
				self.y0 = Some(e[0].y);
				self.x1 = Some(e[1].x);
				self.y1 = Some(e[1].y);
				self.reclip()
			}
			None => {
				self.x0 = None;
				self.y0 = None;
				self.x1 = None;
				self.y1 = None;
				self
			}
		}
	}
}
