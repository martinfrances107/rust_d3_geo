use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::projector::Projector;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::rot::rotate_radians::RotateRadians;
use crate::stream::Streamable;
use crate::Transform;

/// The raw projection.
pub mod azimuthal_equal_area;
/// The raw projection.
pub mod azimuthal_equidistant;
/// The raw projection.
pub mod equirectangular;
/// The raw projection.
pub mod gnomic;
/// The raw projection.
pub mod mercator;
/// The raw projection.
pub mod orthographic;
/// The raw projection.
pub mod stereographic;

/// The default projection builder.
pub mod builder;
/// A specalised builder wrapping the default mecator.
pub mod builder_mercator;
/// Debug and test helper function.
pub mod projection_equal;
/// Projection object.
pub mod projector;
/// Resample based on a given precision.
pub mod resampler;
/// A stream node pipeline stage.
pub mod stream_transform_radians;
/// Scale translate and rotate.
pub mod transform;

/// Helper functions.
mod azimuthal;
/// Helper functions found measuring the extent, width or height.
mod fit_clip;
mod fit_no_clip;

type FitBounds<B, T> = Box<dyn Fn([Coordinate<T>; 2], B) -> B>;

/// Projection type.
pub type RotateTransform<PR, T> =
    Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>;

/// Provides specialization over 'Projection Raw'
///
/// Mercator projections [MercatorTransverseRaw and MercatorRaw]
/// have a extent_transform() for their individual needs.
pub trait TransformExtent {
    /// f64 or f32.
    type T;

    /// Transform the extent stored in MercatorBuilder before
    /// being passing into the base projection builder.
    fn transform_extent(
        self,
        k: Self::T,
        t: Coordinate<Self::T>,
        x0: Self::T,
        y0: Self::T,
        x1: Self::T,
        y1: Self::T,
    ) -> [Coordinate<Self::T>; 2]
    where
        Self::T: CoordFloat;
}

/// Serves as a abstract trait both
/// things that follow the common family of raw projections.
/// and alternatively the less common mercator family of raw projections.
pub trait ProjectionRawBase: Transform {
    /// The default builder.
    type Builder;

    /// Constructs the default projection builder.
    fn builder() -> Self::Builder;
}

pub trait Build
where
    <Self as Build>::T: CoordFloat,
{
    type Drain;
    type I;
    type LB;
    type LC;
    type LU;
    type PCNU;
    type PR;
    type PV;
    type RC;
    type RU;
    type T;
    fn build(
        &self,
    ) -> Projector<
        Self::Drain,
        Self::I,
        Self::LB,
        Self::LC,
        Self::LU,
        Self::PCNU,
        Self::PR,
        Self::PV,
        Self::RC,
        Self::RU,
        Self::T,
    >;
}

/// Controls the projections center point.
///
/// Projection builder sub trait.
pub trait CenterGet {
    /// f64 or f32.
    type T;

    ///  Returns the current center of the projection, which defaults to ⟨0°,0°⟩.
    fn get_center(&self) -> Coordinate<Self::T>
    where
        Self::T: CoordFloat;
}

/// Controls the projections center point.
///
/// Projection builder sub trait.
pub trait CenterSet {
    /// f64 or f32.
    type T;

    /// Sets the projection’s center to the specified center,
    /// a two-element array of longitude and latitude in degrees and returns the projection.
    /// The default is ⟨0°,0°⟩.
    ///
    /// @param point A point specified as a two-dimensional array [longitude, latitude] in degrees.
    ///
    fn center(self, point: &Coordinate<Self::T>) -> Self
    where
        Self::T: CoordFloat;
}

/// Methods to clear or return bounding box.
/// A projection builder sub trait.
pub trait ClipExtentBounded {
    /// f64 or f32
    type T;

    type OutputClear;

    /// Returns a bounding box.
    fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]>
    where
        Self::T: CoordFloat;

    /// clears the bounding box.
    fn clip_extent_clear(self) -> Self::OutputClear
    where
        Self::T: CoordFloat;
}

/// Sets the bounding box.
/// A projection builder sub trait.
pub trait ClipExtentSet {
    /// f64 or f32
    type T;

    type OutputBounded;

    /// Sets the bounding box.
    fn clip_extent(self, extent: &[Coordinate<Self::T>; 2]) -> Self::OutputBounded
    where
        Self::T: CoordFloat;
}

/// Sets the bounding box.
/// A projection builder sub trait.
pub trait ClipExtentAdjust {
    /// f64 or f32
    type T;

    /// Sets the bounding box.
    fn clip_extent_adjust(self, extent: &[Coordinate<Self::T>; 2]) -> Self
    where
        Self::T: CoordFloat;
}

/// Returns or sets the extent of the projection.
/// A projection builder sub trait.
pub trait Fit {
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

    fn fit_extent(self, extent: [[Self::T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;

    ///  Sets the projection’s scale and translate to fit the specified geographic feature in the center of an extent with the given size and top-left corner of [0, 0].
    ///  Returns the projection.
    ///
    ///  Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    ///
    ///  @param size The size of the extent, specified as an array [width, height].
    ///  @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    fn fit_size(self, size: [Self::T; 2], object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;

    /// Similar to fit_size where the height is automatically chosen from
    /// the aspect ratio of object and the given constraint on width.
    fn fit_width(self, w: Self::T, object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;

    /// Similar to fit_size where the width is automatically chosen from
    /// the aspect ratio of object and the given constraint on height.
    fn fit_height(self, h: Self::T, object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;
}

/// Gets the post-projection planar rotation angle.
/// A projection builder sub trait.
pub trait AngleGet {
    /// f64 or f32.
    type T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    fn get_angle(&self) -> Self::T;
}

/// Sets the post-projection planar rotation angle.
/// A projection builder sub trait.
pub trait AngleSet {
    /// f64 or f32.
    type T;

    /// Sets the projection’s post-projection planar rotation angle to the
    /// specified angle in degrees and returns the projection.
    ///
    fn angle(self, angle: Self::T) -> Self;
}

pub trait ClipAngleReset
where
    <Self as ClipAngleReset>::T: AbsDiffEq<Epsilon = Self::T> + CoordFloat + Debug + FloatConst,
{
    type Output;

    ///f64 or f32
    type T;
    fn clip_angle_reset(self) -> Self::Output;
}

pub trait ClipAngleGet
where
    <Self as ClipAngleGet>::T: AbsDiffEq<Epsilon = Self::T> + CoordFloat + Debug + FloatConst,
{
    ///f64 or f32
    type T;

    fn get_clip_angle(&self) -> Self::T;
}

/// Selects the clipping strategy
/// A projection builder sub trait.
pub trait ClipAngleSet {
    type Output;

    ///f64 or f32
    type T;

    ///  Switches the projection builder from antimeridian to circle based clipping.
    fn clip_angle(self, angle: Self::T) -> Self::Output;
}

pub trait ClipAngleAdjust {
    /// Alters the clip angle on a projector builder previously configured to use
    ///  circle based clipping.
    /// A projection builder sub trait.

    /// f64 or f32
    type T;

    /// Given the angle, adjust the projection builder
    /// Must already be set for  cicle based clipping.
    fn clip_angle(self, angle: Self::T) -> Self;
}

pub trait RecenterWithResampling {
    fn reset(self) -> Self;
    fn recenter_with_resampling(self) -> Self;
}

pub trait RecenterNoResampling {
    fn reset(self) -> Self;
    fn recenter_no_resampling(self) -> Self;
}

pub trait ReflectGet {
    /// Returns or sets the x or y reflection.
    /// A projection builder sub trait.

    /// f64 or f32.
    type T;

    /// Is the projection builder set to invert the x-coordinate.
    fn get_reflect_x(&self) -> bool;

    /// Is the projection builder set to invert the x-coordinate.
    fn get_reflect_y(&self) -> bool;
}

pub trait ReflectSet {
    /// f64 or f32.
    type T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x(self, reflect: bool) -> Self
    where
        <Self as ReflectSet>::T: AddAssign
            + AsPrimitive<<Self as ReflectSet>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;

    /// Set the projection builder to invert the y-coordinate.
    fn reflect_y(self, reflect: bool) -> Self
    where
        <Self as ReflectSet>::T: AddAssign
            + AsPrimitive<<Self as ReflectSet>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;
}

/// Given the builder is already set to resample, adjust the precision setting.
/// A projection builder sub trait.
pub trait PrecisionAdjust {
    /// f64 or f32.
    type T;
    ///  Sets the threshold for the projection’s adaptive resampling to the specified value in Pixels and returns the projection.
    ///  This value corresponds to the Douglas–Peucker distance.
    fn precision(self, delta: &Self::T) -> Self;
}

/// Resampling Getter.
///
/// Applies only to projections where the resampling precision has been set.
/// A projection builder sub trait.
pub trait PrecisionGet {
    /// f64 or f32.
    type T;

    ///  Returns the projection’s current resampling precision which defaults to square root of 0.5.
    ///  This value corresponds to the Douglas–Peucker distance.
    fn get_precision(&self) -> Self::T;
}

/// Switch to no resampling.
///
/// A projection builder sub trait.
pub trait PrecisionBypass {
    /// f64 or f32.
    type T;
    type Output;
    fn precision_bypass(self) -> Self::Output;
}

/// Give a resampling precision consume the object and return one that resamples.
///
/// Similar to ResampleAdjust but with conversion.
/// A projection builder sub trait.
pub trait PrecisionSet {
    /// f64 or f32.
    type T;
    type Output;
    ///  Sets the threshold for the projection’s adaptive resampling to the specified value in Pixels and returns the projection.
    ///  This value corresponds to the Douglas–Peucker distance.
    fn precision(self, delta: &Self::T) -> Self::Output;
}

/// Rotation getter and setters.
/// A projection builder sub trait.
pub trait RotateGet {
    /// f64 or f32.
    type T;

    /// Returns the three-axis rotaation.
    fn get_rotate(&self) -> [Self::T; 3];
}

/// Rotation getter and setters.
/// A projection builder sub trait.
pub trait Rotate {
    /// f64 or f32.
    type T;

    ///  Sets the projection’s three-axis rotation to the specified angles, which must be a three-element array of numbers.
    ///
    ///  @param angles  A three-element array of numbers [lambda, phi, gamma] specifying the rotation angles in degrees about each spherical axis.
    ///  (These correspond to yaw, PItch and roll.)
    fn rotate(self, angles: &[Self::T; 3]) -> Self;
}

/// Controls the projections scaling factor.
///
/// Projection builder sub trait.
pub trait ScaleGet {
    /// f32 or f64.
    type T;

    /// Returns the programmed scaling factor.
    fn get_scale(&self) -> Self::T;
}

/// Controls the projections scaling factor.
///
/// Adjust implies that the PCN - is a rectangle and it will be adjusted.
///
/// Projection builder sub trait.
pub trait Scale {
    /// f32 or f64.
    type T;

    ///  Sets the projection’s scale factor to the specified value and returns the projection.
    ///  The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
    ///
    ///  @param scale Scale factor to be used for the projection; the default scale is projection-specific.
    fn scale(self, scale: Self::T) -> Self;
}

/// Controls the projections translation factor.
///
/// Projection builder sub trait.
pub trait TranslateGet {
    /// f32 or f64.
    type T;

    /// Returns the projections translation.
    fn get_translate(&self) -> Coordinate<Self::T>
    where
        Self::T: CoordFloat;
}

/// Controls the projections translation factor.
///
/// Projection builder sub trait.
pub trait Translate {
    /// f32 or f64.
    type T;

    ///  Sets the projection’s translation offset to the specified two-element array [tx, ty] and returns the projection.
    ///  The translation offset determines the PIxel coordinates of the projection’s center. The default translation offset places ⟨0°,0°⟩ at the center of a 960×500 area.
    ///
    ///  @param point A two-element array [tx, ty] specifying the translation offset. The default translation offset of defaults to [480, 250] places ⟨0°,0°⟩ at the center of a 960×500 area.
    fn translate(self, t: &Coordinate<Self::T>) -> Self
    where
        Self::T: CoordFloat;
}
