use geo::CoordFloat;
use geo_types::Coord;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::rot::rotate_radians::RotateRadians;
use crate::stream::Streamable;
use crate::Transform;

/// Builds a specialized (conic) equal area.
pub mod albers;
/// Standard projection with multiple sub projections.
pub mod albers_usa;
/// The raw projection.
pub mod azimuthal_equal_area;
/// The raw projection.
pub mod azimuthal_equidistant;
/// Generate either a [Mercator](crate::projection::mercator::Mercator) or a [```ConicConformal```](crate::projection::conic_conformal::ConicConformal) projection.
pub mod conformal;
/// The raw projection.
pub mod conic_conformal;
/// The raw projection.
pub mod conic_equal_area;
/// The raw projection.
pub mod conic_equidistant;
// No direct Access - Access is through conic_equal_area.
/// The default projection builder.
pub mod builder;
/// The raw projection.
pub mod cylindrical_equal_area;
/// The raw projection.
pub mod equal_earth;
/// The raw projection.
pub mod equidistant;
/// The raw projection.
pub mod equirectangular;
/// The raw projection.
pub mod gnomic;

/// Specific to the `AlbersUSA` projection.
pub mod builder_albers_usa;
/// Cylindrical and Conic projection builder.
pub mod builder_conic;
/// Identity builder.
pub mod builder_identity;
/// A specialized projection builder wrapping the default mercator.
pub mod builder_mercator;
/// A specialized projection builder wrapping the mercator builder.
pub mod builder_mercator_transverse;
/// Enum and generator for "Cylindrical" or "Conic" equal area projection.
pub mod equal_area;
/// Debug and test helper function.
#[cfg(not(tarpaulin_include))]
pub mod equality;
/// The raw projection.
pub mod identity;
/// The raw projection.
pub mod mercator;
/// The raw projection.
pub mod mercator_transverse;
/// The raw projection.
pub mod orthographic;
/// Projection object for `AlbersUsa` projection.
pub mod projector_albers_usa;
/// Projection object.
pub mod projector_common;
/// Projection Identity object.
pub mod projector_identity;
/// Resample based on a given precision.
pub mod resampler;
/// The raw projection.
pub mod stereographic;
/// A stream node path node.
pub mod stream_transform_radians;
/// Scale translate and rotate.
pub mod transform;

/// Helper functions.
mod azimuthal;
/// Helper functions found measuring the extent, width or height.
mod fit_clip;
mod fit_no_clip;
mod fit_reclip;

fn tany(y: f64) -> f64 {
    ((f64::FRAC_PI_2() + y) / 2f64).tan()
}

/// Projection type.
pub type RotateTransform<PR, T> = Compose<RotateRadians<T>, Compose<PR, ScaleTranslateRotate<T>>>;

/// Provides specialization over 'Projection Raw'
///
/// Mercator projections
/// [``MercatorTransverse``](crate::projection::mercator_transverse::MercatorTransverse) and
/// [```Mercator```](crate::projection::mercator::Mercator) have a
/// `extent_transform`() for their individual needs.
pub trait TransformExtent {
    /// f64 or f32.
    type T;

    /// Transform the extent stored in `MercatorBuilder` before being passing
    /// into the base projection builder.
    fn transform_extent(
        &self,
        k: Self::T,
        t: Coord<Self::T>,
        x0: Self::T,
        y0: Self::T,
        x1: Self::T,
        y1: Self::T,
    ) -> [Coord<Self::T>; 2]
    where
        Self::T: CoordFloat;
}

/// Serves as a abstract trait both things that follow the common family of
/// raw projections, and alternatively the less common mercator family of
/// raw projections.
pub trait RawBase: Transform {
    /// The resulting builder type.
    type Builder<DRAIN: Clone>;

    /// Constructs the default projection builder.
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN>;
}

/// Make the constructions of all builders uniform.
pub trait BuilderTrait {
    /// The raw projector.
    type PR;

    /// Constructor.
    fn new(projection_raw: Self::PR) -> Self;
}

/// Output a Projector based on a Builders configuration.
pub trait Build {
    /// The output of the build() call
    type Projector;
    /// Returns a Projector based on a builder configuration.
    fn build(&self) -> Self::Projector;
}

/// Controls the projections center point.
///
/// A Projection builder sub trait.
pub trait CenterGet {
    /// f64 or f32.
    type T;

    ///  Returns the current center of the projection, which defaults to ⟨0°,0°⟩.
    fn center(&self) -> Coord<Self::T>
    where
        Self::T: CoordFloat;
}

/// Sets the projection’s center to the specified center, a two-element
/// array of longitude and latitude in degrees and returns the projection.
/// The default is ⟨0°,0°⟩.
///
/// A Projection builder sub trait.
pub trait CenterSet {
    /// f64 or f32.
    type T;

    /// @param point A point specified as a two-dimensional array
    /// [longitude, latitude] in degrees.
    ///
    fn center_set(&mut self, point: &Coord<Self::T>) -> &mut Self
    where
        Self::T: CoordFloat;
}

/// Returns the clip extent.
///
/// Projection builder sub trait.
pub trait ClipExtentGet {
    /// f64 or f32
    type T;

    /// Returns a bounding box.
    fn clip_extent(&self) -> [Coord<Self::T>; 2]
    where
        Self::T: CoordFloat;
}

/// Methods to clear the bounding box.
///
/// A projection builder sub trait.
pub trait ClipExtentClear {
    /// f64 or f32
    type T;
    /// The resultant builder type.
    type Output;

    /// Clears the bounding box.
    fn clip_extent_clear(&self) -> Self::Output
    where
        Self::T: CoordFloat;
}

/// Sets the bounding box.
///
/// A projection builder sub trait.
pub trait ClipExtentSet {
    /// f64 or f32
    type T;

    /// The resultant builder type.
    type Output;

    /// Sets the bounding box.
    fn clip_extent_set(&self, extent: &[Coord<Self::T>; 2]) -> Self::Output
    where
        Self::T: CoordFloat;
}

/// Adjust an existing the bounding box.
///
/// A projection builder sub trait.
pub trait ClipExtentAdjust {
    /// f64 or f32
    type T;

    /// Sets the bounding box.
    fn clip_extent_adjust(&mut self, extent: &[Coord<Self::T>; 2]) -> &mut Self
    where
        Self::T: CoordFloat;
}

/// Sets the projection’s scale and translate to fit the specified
/// geographic feature in the center of the given extent.
///
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
    /// projection to fit a `GeoJSON` object nj in the center of a 960×500
    /// bounding box with 20 pixels of padding on each side:
    ///
    /// Any clip extent is ignored when determining the new scale and
    /// translate.
    ///
    /// The precision used to compute the bounding box of the given object
    /// is computed at an effective scale of 150.
    ///
    /// @param extent The extent, specified as an array [[x₀, y₀], [x₁,
    ///  y₁]], where x₀ is the left side of the bounding box, y₀ is the
    ///  top, x₁ is the right and y₁ is the bottom.
    ///
    ///  @param object A
    /// geographic feature supported by d3-geo (An extension of `GeoJSON`
    ///   feature).
    #[must_use]
    fn fit_extent(
        &self,
        extent: [Coord<Self::T>; 2],
        object: &impl Streamable<T = Self::T>,
    ) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;

    ///  Sets the projection’s scale and translate to fit the specified
    ///  geographic feature in the center of an extent with the given size
    ///  and top-left corner of [0, 0]. Returns the projection.
    ///
    ///  Any clip extent is ignored when determining the new scale and
    ///  translate. The precision used to compute the bounding box of the
    ///  given object is computed at an effective scale of 150.
    ///
    ///  @param size The size of the extent, specified as an array [width,
    ///  height]. @param object A geographic feature supported by d3-geo
    ///  (An extension of `GeoJSON` feature).
    #[must_use]
    fn fit_size(&self, size: Coord<Self::T>, object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;

    /// Similar to [`fit_size`](Self::fit_size) where the height is automatically chosen from
    /// the aspect ratio of object and the given constraint on width.
    #[must_use]
    fn fit_width(&self, w: Self::T, object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;

    /// Similar to [`fit_size`](Self::fit_size) where the width is automatically chosen from
    /// the aspect ratio of object and the given constraint on height.
    #[must_use]
    fn fit_height(&self, h: Self::T, object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;
}

/// Gets the post-projection planar rotation angle.
///
/// A projection builder sub trait.
pub trait AngleGet {
    /// f64 or f32.
    type T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    fn angle(&self) -> Self::T;
}

/// Sets the post-projection planar rotation angle.
///
/// A projection builder sub trait.
pub trait AngleSet {
    /// f64 or f32.
    type T;

    /// Sets the projection’s post-projection planar rotation angle to the
    /// specified angle in degrees and returns the projection.
    fn angle_set(&mut self, angle: Self::T) -> &mut Self;
}

/// Change the clip strategy from circle to antimeridian.
///
/// A projection builder sub trait.
pub trait ClipAngleReset {
    /// The resultant builder type.
    type Output;

    /// f64 or f32
    type T;

    /// Converts a builder using a circle clipping strategy into one using
    /// the antimerdian strategy.
    fn clip_angle_reset(self) -> Self::Output;
}

/// Clip angle getter.
///
/// API-state design:
/// Note this method is only available on on builders using the
/// circle clipping strategy.
///
/// A projection builder sub trait.
pub trait ClipAngleGet {
    /// f64 or f32
    type T;

    /// Returns the builder clipping angle (radians).
    fn clip_angle(&self) -> Self::T;
}

/// Transforms the builder in one using the circle clipping strategy
///
/// API-state design:
/// Note this method is only available on on builders using the
/// antimeridian clipping strategy.
///
/// A projection builder sub trait.
pub trait ClipAngleSet {
    /// The resultant builder type.
    type Output;

    /// f64 or f32
    type T;

    /// Switches the projection builder from antimeridian to circle based clipping.
    fn clip_angle_set(&self, angle: Self::T) -> Self::Output;
}

/// Alters the clip angle on a projector builder previously configured to
/// use circle based clipping.
///
/// A projection builder sub trait.
pub trait ClipAngleAdjust {
    /// f64 or f32
    type T;

    /// Given the angle, adjust the projection builder Must already be set
    /// for circle based clipping.
    fn clip_angle(&mut self, angle: Self::T) -> &mut Self;
}

/// Returns the x or y reflection.
///
/// A projection builder sub trait.
pub trait ReflectGet {
    /// f64 or f32.
    type T;

    /// Is the projection builder set to invert the x-coordinate.
    fn is_x_reflected(&self) -> bool;

    /// Is the projection builder set to invert the x-coordinate.
    fn is_y_reflected(&self) -> bool;
}

/// Used to set the state of a projector builder.
#[derive(Debug)]
pub enum Reflect {
    /// Invert the sense of the projection.
    Flipped,
    /// Restore the sense of the projection
    Unflipped,
}
/// Sets the x or y reflection.
///
/// A projection builder sub trait.
pub trait ReflectSet {
    /// f64 or f32.
    type T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self;

    /// Set the projection builder to invert the y-coordinate.
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self;
}

/// Given the builder is already set to resample, adjust the precision
/// setting.
///
/// A projection builder sub trait.
pub trait PrecisionAdjust {
    /// f64 or f32.
    type T;
    ///  Sets the threshold for the projection’s adaptive resampling to the
    ///  specified value in Pixels and returns the projection. This value
    ///  corresponds to the Douglas–Peucker distance.
    fn precision_set(&mut self, delta: &Self::T) -> &mut Self;
}

/// Resampling Getter.
///
/// Applies only to projections where the resampling precision has been
/// set. A projection builder sub trait.
///
/// A projection builder sub trait.
pub trait PrecisionGet {
    /// f64 or f32.
    type T;

    ///  Returns the projection’s current resampling precision which
    ///  defaults to square root of 0.5. This value corresponds to the
    ///  Douglas–Peucker distance.
    fn precision(&self) -> Self::T;
}

/// Switch to no resampling.
///
/// A projection builder sub trait.
pub trait PrecisionBypass {
    /// f64 or f32.
    type T;
    /// The resultant builder type.
    type Output;
    /// Switch to no resampling.
    fn precision_bypass(&self) -> Self::Output;
}

/// Give a resampling precision consume the object and return one that
/// resamples.
///
/// A projection builder sub trait.
pub trait PrecisionSet {
    /// f64 or f32.
    type T;
    /// The resultant builder type.
    type Output;
    ///  Sets the threshold for the projection’s adaptive resampling to the
    ///  specified value in Pixels and returns the projection. This value
    ///  corresponds to the Douglas–Peucker distance.
    fn precision_set(&self, delta: &Self::T) -> Self::Output;
}

/// Rotation getter.
///
/// A projection builder sub trait.
pub trait RotateGet {
    /// f64 or f32.
    type T;

    /// Returns the three-axis rotation.
    fn rotate(&self) -> [Self::T; 3];
}

/// Rotation getter and setters.
///
/// A projection builder sub trait.
pub trait RotateSet {
    /// f64 or f32.
    type T;

    ///  Sets two of the  projection’s rotation to the specified angles,
    ///
    ///  @param angles  A two-element array of numbers [lambda, phi]
    ///  specifying the rotation angles in degrees about each
    ///  spherical axis.
    /// (These correspond to yaw, pitch and with roll set to zero.)
    fn rotate2_set(&mut self, angles: &[Self::T; 2]) -> &mut Self;

    ///  Sets the projection’s three-axis rotation to the specified angles,
    ///  which must be a three-element array of numbers.
    ///
    ///  @param angles  A three-element array of numbers [lambda, phi,
    ///  gamma] specifying the rotation angles in degrees about each
    ///  spherical axis. (These correspond to yaw, pitch and roll.)
    fn rotate3_set(&mut self, angles: &[Self::T; 3]) -> &mut Self;
}

/// Controls the projections scaling factor.
///
/// A Projection builder sub trait.
pub trait ScaleGet {
    /// f32 or f64.
    type T;

    /// Returns the programmed scaling factor.
    fn scale(&self) -> Self::T;
}

///  Sets the projection’s scale factor to the specified value and returns
///  the projection. The scale factor corresponds linearly to the distance
///  between projected points; however, absolute scale factors are not
///  equivalent across projections.
///
/// A Projection builder sub trait.
pub trait ScaleSet {
    /// f32 or f64.
    type T;

    ///
    ///  @param scale Scale factor to be used for the projection; the
    ///  default scale is projection-specific.
    fn scale_set(&mut self, scale: Self::T) -> &mut Self;
}

/// Controls the projections translation factor.
///
/// Projection builder sub trait.
pub trait TranslateGet {
    /// f32 or f64.
    type T;

    /// Returns the projections translation.
    fn translate(&self) -> Coord<Self::T>
    where
        Self::T: CoordFloat;
}

///  Sets the projection’s translation offset to the specified two-element
///  array [tx, ty] and returns the projection. The translation offset
///  determines the Pixel coordinates of the projection’s center. The
///  default translation offset places ⟨0°,0°⟩ at the center of a 960×500
///  area.
///
/// A Projection builder sub trait.
pub trait TranslateSet {
    /// f32 or f64.
    type T;

    ///
    ///  @param point A two-element array [tx, ty] specifying the
    ///  translation offset. The default translation offset of defaults to
    ///  [480, 250] places ⟨0°,0°⟩ at the center of a 960×500 area.
    fn translate_set(&mut self, t: &Coord<Self::T>) -> &mut Self
    where
        Self::T: CoordFloat;
}

/// Private traits.

trait Recenter {
    fn recenter(&mut self) -> &mut Self;
}

/// This need to be generic because there are two types of projector.
///
/// Most Projections use a common Projector, the `AlbersUSA` projector is just a container
/// for a collections of projectors.
pub trait Projector {
    /// The endpoint of the stream path.
    type EP;

    /// The act of connecting a drain to the path
    /// creates an object that fundamentally acts as Transformer.
    type Transformer;

    /// Attach a endpoint of a stream path and returns
    /// a transformer to which can be fed geometry objects.
    fn stream(&mut self, drain: &Self::EP) -> Self::Transformer;
}
