pub mod angle;
pub mod builder;
pub mod center_get;
pub mod center_set;
pub mod clip_angle_get;
pub mod clip_angle_set;
pub mod precision_get;
pub mod precision_set;
pub mod reflect;
pub mod scale;
pub mod translate_get;
pub mod translate_set;

use std::marker::PhantomData;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::PointVisible;
use crate::identity::Identity;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::Builder as ProjectionBuilder;
use crate::projection::builder_mercator::Builder as MercatorBuilder;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::ClipExtentBounded;
use crate::projection::Projector;
use crate::projection::TransformExtent;
use crate::projection::TranslateGet;
use crate::projection::TranslateSet;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::Coordinate;
use crate::Transform;

use crate::stream::Unconnected;

/// A wrapper for Projection\Builder which overrides the traits - scale translate and center.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat,
{
	pub pr: PR,
	pub base: ProjectionBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
	pub x0: Option<T>,
	pub y0: Option<T>,
	pub x1: Option<T>,
	pub y1: Option<T>, // post-clip extent
}

impl<DRAIN, PR, T>
	Builder<
		DRAIN,
		InterpolateAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
		Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
		Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// Wrap a default projector and provides mercator specific overrides.
	pub fn new(pr: PR) -> Self {
		let base = ProjectionBuilder::new(
			gen_clip_antimeridian::<
				DRAIN,
				Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>,
				Identity<DRAIN, DRAIN, DRAIN, Unconnected, T>,
				PR,
				ResampleNoClipC<DRAIN, PR, T>,
				ResampleNoClipU<DRAIN, PR, T>,
				T,
			>(),
			pr.clone(),
		);
		Self {
			pr,
			base,
			x0: None,
			y0: None,
			x1: None,
			y1: None,
		}
	}
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
	Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	DRAIN: Clone,
	I: Clone,
	LB: Clone,
	LC: Clone,
	LU: Clone,
	PCNU: Clone,
	PR: Clone,
	PV: Clone,
	RC: Clone,
	RU: Clone,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// Using the currently programmed state output a new projection.
	#[inline]
	pub fn build(&self) -> Projector<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> {
		Projector {
			p_lb: PhantomData::<LB>,
			p_lc: PhantomData::<LC>,
			p_pcnc: PhantomData::<PCNC>,
			cache: None,
			postclip: self.base.postclip.clone(),
			clip: self.base.clip.clone(),
			resample: self.base.resample.clone(),
			rotator: self.base.rotator.clone(),
			project_rotate_transform: self.base.project_rotate_transform.clone(),
			transform_radians: StreamTransformRadians(Unconnected),
		}
	}
}
