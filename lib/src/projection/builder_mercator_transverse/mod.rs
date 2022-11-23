/// A builder getter.
pub mod center_get;
/// A builder tansform.
pub mod center_set;
/// A builder tansform.
pub mod clip_extent_adjust;
/// A builder tansform.
pub mod clip_extent_clear;
/// A builder getter.
pub mod clip_extent_get;
/// A builder tansform.
pub mod fit;
/// A builder tansform.
pub mod precision_bypass;
/// A builder tansform.
pub mod reclip;
/// A builder getter.
pub mod rotate_get;
/// A builder tansform.
pub mod rotate_set;
/// A builder getter.
pub mod scale_get;
/// A builder tansform.
pub mod scale_set;
/// A builder tansform.
pub mod translate_adjust;
/// A builder getter.
pub mod translate_get;

/// Builder shorthand notations.
pub mod types;

use std::marker::PhantomData;

use approx::AbsDiffEq;
use derivative::Derivative;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder_mercator::Builder as ProjectionMercatorBuilder;
use crate::projection::Build;
use crate::projection::Projector;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use self::types::BuilderMercatorTransverseAntimeridianResampleClip;

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
    /// The type this builder wraps.
    pub base: ProjectionMercatorBuilder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>,
}

impl<DRAIN, PR, T> BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    /// Wrap a default projector and provides mercator specific overrides.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as constants will always be converted into T.
    pub fn new(pr: PR) -> Self {
        let mut base = ProjectionMercatorBuilder::new(pr);
        base.rotate3_set(&[T::zero(), T::zero(), T::from(90).unwrap()])
            .scale_set(T::from(159.155).unwrap());

        Self { base }
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
