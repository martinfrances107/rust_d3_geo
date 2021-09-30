use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::clip::Clip;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::compose::Compose;
use crate::data_object::DataObject;
use crate::path::bounds_stream::BoundsStream;
use crate::rotation::rotate_radians::RotateRadians;
use crate::stream::Stream;
use crate::Transform;

use self::str::scale_translate_rotate::ScaleTranslateRotate;
use projection::Projection;
use resample::ResampleNode;
use stream_node::StreamNode;
use stream_node_factory::StreamNodeFactory;

/// The raw projection.
pub mod azimuthal_equal_area;
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

/// Sub Traits
pub mod builder_trait;

/// Helper functions.
pub mod azimuthal;
/// The default projection builder.
pub mod builder;
/// A specalised builder wrapping the default mecator.
pub mod mercator_builder;
/// Projection object.
pub mod projection;
/// Debug and test helper function.
pub mod projection_equal;
/// Scale translate and rotate.
pub mod str;
/// Stream node pipeline stage.
pub mod stream_node;
/// Generates stream node objects.
pub mod stream_node_factory;
/// A stream node pipeline stage.
pub mod stream_transform_radians;

/// Helper functions found measuring the extent, width or height.
mod fit;

mod resample;

/// Projection type.
pub type PostClipFactory<DRAIN, L, PR, PV, T> = StreamNodeFactory<
    PostClipNode<DRAIN, T>,
    StreamNode<
        Clip<L, PV, ResampleNode<PR, PostClipNode<DRAIN, T>, T>, T>,
        ResampleNode<PR, PostClipNode<DRAIN, T>, T>,
        T,
    >,
    T,
>;

/// Projection type.
pub type RotateFactory<DRAIN, L, PR, PV, T> = StreamNodeFactory<
    RotateRadians<T>,
    StreamNode<
        Clip<L, PV, ResampleNode<PR, PostClipNode<DRAIN, T>, T>, T>,
        ResampleNode<PR, PostClipNode<DRAIN, T>, T>,
        T,
    >,
    T,
>;

/// Projection type.
pub type RotateTransformFactory<DRAIN, L, PR, PV, T> = StreamNodeFactory<
    Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,
    StreamNode<
        Clip<L, PV, ResampleNode<PR, PostClipNode<DRAIN, T>, T>, T>,
        ResampleNode<PR, PostClipNode<DRAIN, T>, T>,
        T,
    >,
    T,
>;

/// Projection Raw.
pub trait Raw<T>: Clone + Debug + Default + Transform<T = T>
where
    <Self as Transform>::T: CoordFloat,
{
    /// The default builder.
    type Builder;
    /// f32 or f64.
    type T;
    /// Constructs the default projection builder.
    fn builder() -> Self::Builder;
}

trait Builder
where
    <Self as Builder>::Drain: Stream<T = <Self as Builder>::T>,
    <Self as Builder>::L: Line,
    <Self as Builder>::PR: Raw<Self::T>,
    <Self as Builder>::PV: PointVisible<T = Self::T>,
    <Self as Builder>::T: CoordFloat + FloatConst,
{
    type Drain;
    type L;
    type PR;
    type PV;
    type T;
    fn build(s: Self::PR) -> Projection<Self::Drain, Self::L, Self::PR, Self::PV, Self::T>;
}

/// Controls the projections center point.
///
/// Projection builder sub trait.
pub trait Center // where
//     T: CoordFloat,
{
    /// f64 or f32
    type T;

    ///  Returns the current center of the projection, which defaults to ⟨0°,0°⟩.
    fn get_center(&self) -> Coordinate<Self::T>
    where
        Self::T: CoordFloat;

    /// Sets the projection’s center to the specified center,
    /// a two-element array of longitude and latitude in degrees and returns the projection.
    /// The default is ⟨0°,0°⟩.
    ///
    /// @param point A point specified as a two-dimensional array [longitude, latitude] in degrees.
    ///
    fn center(self, point: Coordinate<Self::T>) -> Self
    where
        Self::T: CoordFloat;
}

/// Returns or sets the bounding box.
/// A projection builder sub trait.
pub trait ClipExtent {
    /// f64 or f32
    type T;

    /// Returns a bounding box.
    fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]>
    where
        Self::T: CoordFloat;

    /// Sets the bounding box.
    fn clip_extent(self, extent: Option<[Coordinate<Self::T>; 2]>) -> Self
    where
        Self::T: CoordFloat;
}

/// Returns or sets the extent of the projection.
/// A projection builder sub trait.
pub trait Fit {
    /// f64 or f32
    type T;

    ///   Sets the projection’s scale and translate to fit the specified geographic feature in the center of the given extent.
    ///   Returns the projection.
    ///
    ///   Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    ///
    ///   @param extent The extent, specified as an array [[x₀, y₀], [x₁, y₁]], where x₀ is the left side of the bounding box, y₀ is the top, x₁ is the right and y₁ is the bottom.
    ///   @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    fn fit_extent(self, extent: [[Self::T; 2]; 2], object: DataObject<Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;

    ///  Sets the projection’s scale and translate to fit the specified geographic feature in the center of an extent with the given size and top-left corner of [0, 0].
    ///  Returns the projection.
    ///
    ///  Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    ///
    ///  @param size The size of the extent, specified as an array [width, height].
    ///  @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    fn fit_size(self, size: [Self::T; 2], object: DataObject<Self::T>) -> Self
    where
        Self::T: AsPrimitive<Self::T> + CoordFloat;
}

/// Returns or sets the post-projection planar rotation angle.
/// A projection builder sub trait.
pub trait Angle {
    /// f64 or f32
    type T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    fn get_angle(&self) -> Self::T;

    /// Sets the projection’s post-projection planar rotation angle to the
    /// specified angle in degrees and returns the projection.
    ///
    fn angle(self, angle: Self::T) -> Self;
}

/// Returns or sets the x or y reflection.
/// A projection builder sub trait.
pub trait Reflect {
    /// f64 or f32
    type T;

    /// Is the projection builder set to invert the x-coordinate.
    fn get_reflect_x(&self) -> bool;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x(self, reflect: bool) -> Self
    where
        // <Self as Reflect>::PR: Transform<T = <Self as Reflect>::T>,
        <Self as Reflect>::T: AddAssign
            + AsPrimitive<<Self as Reflect>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;

    /// Is the projection builder set to invert the x-coordinate.
    fn get_reflect_y(&self) -> bool;

    /// Set the projection builder to invert the y-coordinate.
    fn reflect_y(self, reflect: bool) -> Self
    where
        // <Self as Reflect>::PR: Transform<T = <Self as Reflect>::T>,
        <Self as Reflect>::T: AddAssign
            + AsPrimitive<<Self as Reflect>::T>
            + CoordFloat
            + Debug
            + Display
            + FloatConst;
}

/// Generates elements of the projection stream pipeline.
pub trait NodeFactory
where
    <Self as NodeFactory>::T: CoordFloat,
{
    /// The resultant node type.
    type Node;
    /// The downstream node.
    type Sink;
    /// f32 or f64.
    type T;

    /// Combine the sink with the proto-node and output a StreamNode.
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> Self::Node;
}

/// Resampling getter and setters.
pub trait Precision {
    /// f64 or f32
    type T;

    ///  Returns the projection’s current resampling precision which defaults to square root of 0.5.
    ///  This value corresponds to the Douglas–Peucker distance.
    fn get_precision(&self) -> Self::T;

    ///  Sets the threshold for the projection’s adaptive resampling to the specified value in Pixels and returns the projection.
    ///  This value corresponds to the Douglas–Peucker distance.
    fn precision(self, delta: &Self::T) -> Self;
}

/// Rotation getter and setters.
pub trait Rotate {
    /// f64 or f32
    type T;

    /// Returns the three-axis rotaation.
    fn get_rotate(&self) -> [Self::T; 3];

    ///  Sets the projection’s three-axis rotation to the specified angles, which must be a two- or three-element array of numbers.
    ///
    ///  @param angles  A two- or three-element array of numbers [lambda, phi, gamma] specifying the rotation angles in degrees about each spherical axis.
    ///  (These correspond to yaw, PItch and roll.) If the rotation angle gamma is omitted, it defaults to 0.
    ///
    fn rotate(self, angles: [Self::T; 3]) -> Self;
}

/// Controls the projections scaling factor.
///
/// Projection builder sub trait.
pub trait Scale {
    /// f32 or f64.
    type T;

    /// Returns the programmed scaling factor.
    fn get_scale(&self) -> Self::T;

    ///  Sets the projection’s scale factor to the specified value and returns the projection.
    ///  The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
    ///
    ///  @param scale Scale factor to be used for the projection; the default scale is projection-specific.
    fn scale(self, scale: Self::T) -> Self;
}

/// Controls the projections translation factor.
///
/// Projection builder sub trait.
pub trait Translate {
    /// f32 or f64.
    type T;

    /// Returns the projections translation.
    fn get_translate(&self) -> Coordinate<Self::T>
    where
        Self::T: CoordFloat;

    ///  Sets the projection’s translation offset to the specified two-element array [tx, ty] and returns the projection.
    ///  The translation offset determines the PIxel coordinates of the projection’s center. The default translation offset places ⟨0°,0°⟩ at the center of a 960×500 area.
    ///
    ///  @param point A two-element array [tx, ty] specifying the translation offset. The default translation offset of defaults to [480, 250] places ⟨0°,0°⟩ at the center of a 960×500 area.
    fn translate(self, t: &Coordinate<Self::T>) -> Self
    where
        Self::T: CoordFloat;
}
