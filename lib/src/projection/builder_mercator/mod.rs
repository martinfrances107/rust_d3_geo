pub mod angle;
pub mod builder;
pub mod center_get;
pub mod center_set;
pub mod clip_angle_get;
pub mod clip_angle_set;
pub mod clip_bounded;
pub mod clip_extent_adjust;
pub mod clip_extent_set;
pub mod precision_adjust;
pub mod precision_bypass;
pub mod precision_get;
pub mod precision_set;
pub mod reclip;
pub mod reclip_adjust;
pub mod reflect_get;
pub mod reflect_set;
pub mod rotate_get;
pub mod rotate_set;
pub mod scale_adjust;
pub mod scale_get;
pub mod scale_set;
pub mod translate_adjust;
pub mod translate_get;
pub mod translate_set;

use geo::Coordinate;
use std::fmt::Debug;
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
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::Builder as ProjectionBuilder;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Projector;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

trait Reclip {
	type Output;
	fn reclip(self) -> Self::Output;
}

trait ReclipAdjust {
	fn reclip_adjust(self) -> Self;
}

/// A wrapper for Projection\Builder which overrides the traits - scale translate and center.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat,
{
	pub pr: PR,
	pub base: ProjectionBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
	pub extent: Option<[Coordinate<T>; 2]>, // post-clip extent
}

impl<DRAIN, PR, T>
	Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Debug + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// Wrap a default projector and provides mercator specific overrides.
	pub fn new(pr: PR) -> Self {
		let base = ProjectionBuilder::new(
			gen_clip_antimeridian::<
				DRAIN,
				NoClipC<DRAIN, T>,
				NoClipU<DRAIN, T>,
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
			extent: None,
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
	PCNU: Clone + Debug,
	PR: Clone,
	PV: Clone,
	RC: Clone,
	RU: Debug + Clone,
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
