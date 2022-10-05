pub mod angle;
pub mod builder;
pub mod center_get;
pub mod center_set;
pub mod clip_angle_set;
pub mod clip_extent_adjust;
pub mod clip_extent_clear;
pub mod clip_extent_get;
pub mod clip_extent_set;
pub mod fit;
pub mod fit_reclip;
pub mod precision_adjust;
pub mod precision_bypass;
pub mod precision_get;
pub mod precision_set;
pub mod reclip;
/// Reclip is private beacuse it is only used to intialise the builder.
mod reclip_convert;
pub mod reflect_get;
pub mod reflect_set;
pub mod rotate_get;
pub mod rotate_set;
pub mod scale_adjust;
pub mod scale_get;
pub mod translate_adjust;
pub mod translate_get;
pub mod translate_reclip;
pub mod types;

use std::marker::PhantomData;

use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::ResampleNoPCNC;
use crate::projection::builder::template::ResampleNoPCNU;
use crate::projection::builder::Builder as ProjectionBuilder;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;
use crate::projection::Projector;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::stream::Unconnected;
use crate::Transform;

use self::types::BuilderMercatorAntimeridianResampleClip;

use super::builder::template::ResamplePCNC;
use super::ClipExtentSet;
use super::TransformExtent;

/// Returns or sets the extent of the projection.
/// A projection builder sub trait.
/// This trait is useful only for mercator projection.
/// Here  centering, scaling and translate all end in a reclip.
/// That is all involve a tranformation of the PCN
/// specifcally a Identity struct to a Rectangle struct.
pub trait FitReclip {
    type Output;
    /// f64 or f32.
    type T;

    /// Sets the projection’s scale and translate to fit the specified
    /// geographic feature in the center of the given extent.
    ///
    /// Returns the projection.
    ///
    /// For example, to scale and translate the New Jersey State Plane
    /// projection to fit a GeoJSON object nj in the center of a 960×500
    /// bounding box with 20 pixels of padding on each side:
    ///
    /// Any clip extent is ignored when determining the new scale and
    /// translate.
    ///
    /// The precision used to compute the bounding box of the given object is
    /// computed at an effective scale of 150.
    ///
    /// @param extent The extent, specified as an array [[x₀, y₀], [x₁, y₁]],
    ///  where x₀ is the left side of the bounding box, y₀ is the top,
    ///  x₁ is the right and y₁ is the bottom.
    /// @param object A geographic feature supported by d3-geo
    ///   (An extension of GeoJSON feature).
    fn fit_extent_reclip(
        &self,
        extent: [[Self::T; 2]; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> Self::Output;

    ///  Sets the projection’s scale and translate to fit the specified geographic feature in the center of an extent with the given size and top-left corner of [0, 0].
    ///  Returns the projection.
    ///
    ///  Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    ///
    ///  @param size The size of the extent, specified as an array [width, height].
    ///  @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    fn fit_size_reclip(
        &self,
        size: [Self::T; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> Self::Output;

    /// Similar to fit_size where the width is automatically chosen from
    /// the aspect ratio of object and the given constraint on height.
    fn fit_width_reclip(&self, h: Self::T, object: &impl Streamable<T = Self::T>) -> Self::Output;

    /// Similar to fit_size where the height is automatically chosen from
    /// the aspect ratio of object and the given constraint on height.
    fn fit_height_reclip(&self, h: Self::T, object: &impl Streamable<T = Self::T>) -> Self::Output;
}

/// This trait is useful only for mercator projection.
/// Here  centering, scaling and translate all end in a reclip.
/// That is all involve a tranformation of the PCN
/// specifcally a Identity struct to a Rectangle struct.
pub trait ScaleReclip {
    /// Output type where the PCN is set to Rectangle.
    type Output;

    /// f32 or f64.
    type T;

    ///  Sets the projection’s scale factor to the specified value and returns the projection.
    ///  The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
    ///
    ///  @param scale Scale factor to be used for the projection; the default scale is projection-specific.
    fn scale_reclip(self, scale: Self::T) -> Self::Output;
}

/// Controls the projections translation factor.
///
/// Projection builder sub trait.
pub trait TranslateReclip {
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

pub trait Reclip {
    fn reclip(&mut self) -> &mut Self;
}

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
    pub pr: PR,
    pub base: ProjectionBuilder<CLIPC, CLIPU, DRAIN, PCNU, PR, RC, RU, T>,
    pub extent: Option<[Coordinate<T>; 2]>, // post-clip extent
}

impl<DRAIN, PR, T> BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + Default + FloatConst,
{
    /// Wrap a default projector and provides mercator specific overrides.
    pub fn new(pr: PR) -> Self {
        let base = ProjectionBuilder::new(pr.clone());
        // Dummy clip values here will be overriten by the following reclip.
        let base = base.clip_extent_set(&[
            Coordinate {
                x: T::zero(),
                y: T::zero(),
            },
            Coordinate {
                x: T::zero(),
                y: T::zero(),
            },
        ]);

        let mut out = Self {
            p_clipc: PhantomData::<ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<ResamplePCNC<DRAIN, PR, T>>,
            pr,
            base,
            extent: None,
        };
        out.reclip();
        out
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
            // p_drain: PhantomData::<DRAIN>,
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
