pub mod angle;
pub mod builder;
pub mod center_get;
pub mod center_set;
pub mod clip_angle_get;
pub mod clip_angle_set;
pub mod clip_bounded;
pub mod clip_extent_adjust;
pub mod clip_extent_set;
pub mod fit_adjust;
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
pub mod types;

use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::Builder as ProjectionBuilder;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Projector;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

/// This trait is useful only for mercator projection.
/// Here  centering, scaling and trasnlate all end in a reclip.
/// That is all involve a tranformation of the PCN
/// specifcally a Identity struct to a Rectangle struct.
pub trait ScaleSet {
	/// Output type where the PCN is set to Rectangle.
	type Output;

	/// f32 or f64.
	type T;

	///  Sets the projection’s scale factor to the specified value and returns the projection.
	///  The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
	///
	///  @param scale Scale factor to be used for the projection; the default scale is projection-specific.
	fn scale(self, scale: Self::T) -> Self::Output;
}

/// Controls the projections translation factor.
///
/// Projection builder sub trait.
pub trait TranslateSet {
	type Output;
	/// f32 or f64.
	type T;

	///  Sets the projection’s translation offset to the specified two-element array [tx, ty] and returns the projection.
	///  The translation offset determines the PIxel coordinates of the projection’s center. The default translation offset places ⟨0°,0°⟩ at the center of a 960×500 area.
	///
	///  @param point A two-element array [tx, ty] specifying the translation offset. The default translation offset of defaults to [480, 250] places ⟨0°,0°⟩ at the center of a 960×500 area.
	fn translate(self, t: &Coordinate<Self::T>) -> Self::Output
	where
		Self::T: CoordFloat;
}

trait Reclip {
	type Output;
	fn reclip(self) -> Self::Output;
}

trait ReclipAdjust {
	fn reclip_adjust(self) -> Self;
}

/// A wrapper over Projection\Builder which overrides the traits - scale translate and center.
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

impl<DRAIN, PR, T> BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
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
