// pub mod angle;
pub mod center_get;
pub mod center_set;
// pub mod clip_angle_set;
pub mod clip_extent_adjust;
pub mod clip_extent_clear;
pub mod clip_extent_get;
// pub mod fit;
// pub mod fit_reclip;
// pub mod precision_adjust;
pub mod precision_bypass;
// pub mod precision_get;
// pub mod precision_set;
pub mod reclip;
// pub mod reflect_get;
// pub mod reflect_set;
pub mod rotate_get;
pub mod rotate_set;
// pub mod scale_adjust;
// pub mod scale_get;
pub mod translate_adjust;
// pub mod translate_get;
mod scale_set;
pub mod types;

use std::marker::PhantomData;

use approx::AbsDiffEq;
use derivative::Derivative;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::projection::builder_mercator::Builder as ProjectionMercatorBuilder;
use crate::projection::Build;
use crate::projection::Projector;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use self::types::BuilderMercatorTransverseAntimeridianResampleClip;

use super::builder::template::ResamplePCNC;
use super::stream_transform_radians::StreamTransformRadians;
use super::RotateSet;
use super::ScaleSet;
use super::TransformExtent;

/// A wrapper over Projection\Builder which overrides the traits - scale translate and center.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    p_clipc: PhantomData<CLIPC>,
    p_drain: PhantomData<DRAIN>,
    p_rc: PhantomData<RC>,
    // pub pr: PR,
    pub base: ProjectionMercatorBuilder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>,
}

impl<DRAIN, PR, T> BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    /// Wrap a default projector and provides mercator specific overrides.
    pub fn new(pr: PR) -> Self {
        let mut base = ProjectionMercatorBuilder::new(pr);
        base.rotate_set(&[T::zero(), T::zero(), T::from(90).unwrap()]);
        base.scale_set(T::from(159.155).unwrap());

        Self {
            p_clipc: PhantomData::<ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<ResamplePCNC<DRAIN, PR, T>>,
            base,
        }
    }
}

impl<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> Build
    for Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PCNU: Clone,
    PR: Clone,
    RU: Clone,
    T: CoordFloat,
{
    type ClipC = CLIPC;
    type ClipU = CLIPU;
    type Drain = DRAIN;
    type PCNU = PCNU;
    type PR = PR;
    type RC = RC;
    type RU = RU;
    type T = T;

    /// Using the currently programmed state output a new projection.
    #[inline]
    fn build(&self) -> Projector<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T> {
        Projector {
            p_rc: PhantomData::<RC>,
            cache: None,
            postclip: self.base.base.postclip.clone(),
            clip: self.base.base.clip.clone(),
            resample: self.base.base.resample.clone(),
            rotator: self.base.base.rotator.clone(),
            project_rotate_transform: self.base.base.project_rotate_transform.clone(),
            transform_radians: StreamTransformRadians(Unconnected),
        }
    }
}
