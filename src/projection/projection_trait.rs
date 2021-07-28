use geo::Coordinate;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

// use crate::data_object::DataObject;
use crate::stream::Stream;
use crate::Transform;

use super::center::Center;
use super::clip_extent::ClipExtent;
use super::projection::Projection;
// use super::projection::StreamOrValueMaybe;
use super::scale::Scale;
use super::translate::Translate;
// use crate::compose::Compose;

// use super::scale_translate_rotate::ScaleTranslateRotateEnum;
// use super::ProjectionRawTrait;
use crate::projection::stream_transform::StreamTransform;
use crate::projection::stream_transform_radians::StreamTransformRadians;
pub trait ProjectionTrait<'a>: Center + ClipExtent + Scale + Translate
// Rc<<Self as ProjectionTrait<'a>>::PR>: ProjectionRawTrait,
where
    // Rc<<Self as ProjectionTrait<'a>>::PR>:
    //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
    <Self as ProjectionTrait<'a>>::PR:
        Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
    // <Self as ProjectionTrait<'a>>::SD:
    //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
    <Self as ProjectionTrait<'a>>::T: AddAssign
        + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
        + Debug
        + Display
        + Float
        + FloatConst,
{
    type PR;
    type T;
    type C;
    // type SD;
    // /**
    //  * Returns a new array [x, y] (tyPIcally in PIxels) representing the projected point of the given point.
    //  * The point must be specified as a two-element array [longitude, latitude] in degrees.
    //  * May return null if the specified point has no defined projected position, such as when the point is outside the clipPIng bounds of the projection.
    //  *
    //  * @param point A point specified as a two-dimensional array [longitude, latitude] in degrees.
    //  */
    // (point: [number, number]): [number, number] | null;

    // /**
    //  * Returns the current spherical clipPIng function.
    //  * Pre-clipPIng occurs in geographic coordinates. Cutting along the antimeridian line,
    //  * or clipPIng along a small circle are the most common strategies.
    //  */
    // fn get_preclip(&self) -> ClipNode<T>;

    // /**
    //  * Sets the projection’s spherical clipPIng to the specified function and returns the projection.
    //  * Pre-clipPIng occurs in geographic coordinates. Cutting along the antimeridian line, or clipPIng along a small circle are the most common strategies.
    //  *
    //  * @param preclip A spherical clipPIng function. ClipPIng functions are implemented as transformations of a projection stream.
    //  * Pre-clipPIng operates on spherical coordinates, in radians.
    //  */
    // fn preclip(&mut self, preclip: ClipNode<T>);

    // /**
    //  * Returns the current cartesian clipPIng function.
    //  * Post-clipPIng occurs on the plane, when a projection is bounded to a certain extent such as a rectangle.
    //  */
    // fn get_postclip(&self) -> Option<Box<dyn GeoStream>>;
    // /**
    //  * Sets the projection’s cartesian clipPIng to the specified function and returns the projection.
    //  *
    //  * @param postclip A cartesian clipPIng function. ClipPIng functions are implemented as transformations of a projection stream.
    //  * Post-clipPIng operates on planar coordinates, in PIxels.
    //  */
    // fn postclip(&mut self, postclip: StreamProcessor<T>);

    // /**
    //  * Switches to antimeridian cutting rather than small-circle clipPIng.
    //  * See also projection.preclip, d3.geoClipAntimeridian, d3.geoClipCircle.
    //  *
    //  * @param angle Set to null to switch to antimeridian cutting.
    //  */
    fn clip_angle(
        self,
        angle: <Self as ProjectionTrait<'a>>::T,
        // angle: StreamOrValueMaybe<<Self as ProjectionTrait<'a>>::T>,
    ) -> Projection<'a, Self::PR, <Self as ProjectionTrait<'a>>::T>;
    // where
    //     // Rc<<Self as ProjectionTrait<'a>>::PR>:
    //     //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
    //     <Self as ProjectionTrait<'a>>::PR:
    //         Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
    //     // <Self as ProjectionTrait<'a>>::SD: StreamDst,
    //     // <Self as ProjectionTrait<'a>>::SD: StreamDst,
    //     <Self as ProjectionTrait<'a>>::SD:
    //         Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
    //     <Self as ProjectionTrait<'a>>::T: AddAssign
    //         + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
    //         + Debug
    //         + Default
    //         + Display
    //         + Float
    //         + FloatConst;

    // /**
    //  * Sets the projection’s clipPIng circle radius to the specified angle in degrees and returns the projection.
    //  * Small-circle clipPIng is independent of viewport clipPIng via projection.clipExtent.
    //  *
    //  * See also projection.preclip, d3.geoClipAntimeridian, d3.geoClipCircle.
    //  *
    //  * @param angle Angle in degrees.
    //  */
    // clipAngle(angle: number): this;

    // /**
    //  * Returns the current viewport clip extent which defaults to null.
    //  */
    // clipExtent(): [[number, number], [number, number]] | null;
    // /**
    //  * Sets the clip extent to null and returns the projection.
    //  * With a clip extent of null, no viewport clipPIng is performed.
    //  *
    //  * Viewport clipPIng is independent of small-circle clipPIng via projection.clipAngle.
    //  *
    //  * See also projection.postclip, d3.geoClipRectangle.
    //  *
    //  * @param extent Set to null to disable viewport clipPIng.
    //  */
    // clipExtent(extent: null): this;
    // /**
    //  * Sets the projection’s viewport clip extent to the specified bounds in PIxels and returns the projection.
    //  * The extent bounds are specified as an array [[x₀, y₀], [x₁, y₁]], where x₀ is the left-side of the viewport, y₀ is the top, x₁ is the right and y₁ is the bottom.
    //  *
    //  * Viewport clipPIng is independent of small-circle clipPIng via projection.clipAngle.
    //  *
    //  * See also projection.postclip, d3.geoClipRectangle.
    //  *
    //  * @param extent The extent bounds are specified as an array [[x₀, y₀], [x₁, y₁]], where x₀ is the left-side of the viewport, y₀ is the top, x₁ is the right and y₁ is the bottom.
    //  */
    // clipExtent(extent: [[number, number], [number, number]]): this;

    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic feature in the center of the given extent.
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param extent The extent, specified as an array [[x₀, y₀], [x₁, y₁]], where x₀ is the left side of the bounding box, y₀ is the top, x₁ is the right and y₁ is the bottom.
    //  * @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    //  */
    // fitExtent(extent: [[number, number], [number, number]], object: ExtendedFeature): this;
    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic feature collection in the center of the given extent.
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param extent The extent, specified as an array [[x₀, y₀], [x₁, y₁]], where x₀ is the left side of the bounding box, y₀ is the top, x₁ is the right and y₁ is the bottom.
    //  * @param object A geographic feature collection supported by d3-geo (An extension of GeoJSON feature collection).
    //  */
    // fitExtent(extent: [[number, number], [number, number]], object: ExtendedFeatureCollection): this;
    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic geometry object in the center of the given extent.
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param extent The extent, specified as an array [[x₀, y₀], [x₁, y₁]], where x₀ is the left side of the bounding box, y₀ is the top, x₁ is the right and y₁ is the bottom.
    //  * @param object A GeoJson Geometry Object or GeoSphere object supported by d3-geo (An extension of GeoJSON).
    //  */
    // fn fit_extent(
    //     self,
    //     extent: [<Self as ProjectionTrait<'a>>::C; 2],
    //     object: DataObject<<Self as ProjectionTrait<'a>>::T>,
    // ) -> Projection<'a, Self::PR, Self::SD, <Self as ProjectionTrait<'a>>::T>
    // where
    //     // Rc<<Self as ProjectionTrait<'a>>::PR>:
    //     //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
    //     <Self as ProjectionTrait<'a>>::PR:
    //         Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
    //     <Self as ProjectionTrait<'a>>::SD:
    //         Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
    //     <Self as ProjectionTrait<'a>>::T: AddAssign
    //         + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
    //         + Debug
    //         + Default
    //         + Display
    //         + Float
    //         + FloatConst;
    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic geometry collection in the center of the given extent.
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param extent The extent, specified as an array [[x₀, y₀], [x₁, y₁]], where x₀ is the left side of the bounding box, y₀ is the top, x₁ is the right and y₁ is the bottom.
    //  * @param object A geographic geometry collection supported by d3-geo (An extension of GeoJSON geometry collection).
    //  */
    // fitExtent(extent: [[number, number], [number, number]], object: ExtendedGeometryCollection): this;

    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic feature in the center of an extent with the given size and top-left corner of [0, 0].
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param size The size of the extent, specified as an array [width, height].
    //  * @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    //  */
    // fitSize(size: [number, number], object: ExtendedFeature): this;
    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic feature collection in the center of an extent with the given size and top-left corner of [0, 0].
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param size The size of the extent, specified as an array [width, height].
    //  * @param object A geographic feature collection supported by d3-geo (An extension of GeoJSON feature collection).
    //  */
    // fitSize(size: [number, number], object: ExtendedFeatureCollection): this;
    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic geometry object in the center of an extent with the given size and top-left corner of [0, 0].
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param size The size of the extent, specified as an array [width, height].
    //  * @param object A GeoJson Geometry Object or GeoSphere object supported by d3-geo (An extension of GeoJSON).
    //  */
    // fitSize(size: [number, number], object: GeoGeometryObjects): this;
    // /**
    //  * Sets the projection’s scale and translate to fit the specified geographic geometry collection in the center of an extent with the given size and top-left corner of [0, 0].
    //  * Returns the projection.
    //  *
    //  * Any clip extent is ignored when determining the new scale and translate. The precision used to compute the bounding box of the given object is computed at an effective scale of 150.
    //  *
    //  * @param size The size of the extent, specified as an array [width, height].
    //  * @param object A geographic geometry collection supported by d3-geo (An extension of GeoJSON geometry collection).
    //  */
    // fitSize(size: [number, number], object: ExtendedGeometryCollection): this;

    // /**
    //  * A convenience method for projection.fitSize where the height is automatically chosen from the aspect ratio of object and the given constraint on width.
    //  *
    //  * @param width The width of the extent.
    //  * @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    //  */
    // fitWidth(width: number, object: ExtendedFeature): this;
    // /**
    //  * A convenience method for projection.fitSize where the height is automatically chosen from the aspect ratio of object and the given constraint on width.
    //  *
    //  * @param width The width of the extent.
    //  * @param object A GeoJson Geometry Object or GeoSphere object supported by d3-geo (An extension of GeoJSON).
    //  */
    // fitWidth(width: number, object: ExtendedFeatureCollection): this;
    // /**
    //  * A convenience method for projection.fitSize where the height is automatically chosen from the aspect ratio of object and the given constraint on width.
    //  *
    //  * @param width The width of the extent.
    //  * @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    //  */
    // fitWidth(width: number, object: GeoGeometryObjects): this;
    // /**
    //  * A convenience method for projection.fitSize where the height is automatically chosen from the aspect ratio of object and the given constraint on width.
    //  *
    //  * @param width The width of the extent.
    //  * @param object A geographic geometry collection supported by d3-geo (An extension of GeoJSON geometry collection).
    //  */
    // fitWidth(width: number, object: ExtendedGeometryCollection): this;

    // /**
    //  * A convenience method for projection.fitSize where the width is automatically chosen from the aspect ratio of object and the given constraint on height.
    //  *
    //  * @param height The height of the extent.
    //  * @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    //  */
    // fitHeight(height: number, object: ExtendedFeature): this;
    // /**
    //  * A convenience method for projection.fitSize where the width is automatically chosen from the aspect ratio of object and the given constraint on height.
    //  *
    //  * @param height The height of the extent.
    //  * @param object A GeoJson Geometry Object or GeoSphere object supported by d3-geo (An extension of GeoJSON).
    //  */
    // fitHeight(height: number, object: ExtendedFeatureCollection): this;
    // /**
    //  * A convenience method for projection.fitSize where the width is automatically chosen from the aspect ratio of object and the given constraint on height.
    //  *
    //  * @param height The height of the extent.
    //  * @param object A geographic feature supported by d3-geo (An extension of GeoJSON feature).
    //  */
    // fitHeight(height: number, object: GeoGeometryObjects): this;
    // /**
    //  * A convenience method for projection.fitSize where the width is automatically chosen from the aspect ratio of object and the given constraint on height.
    //  *
    //  * @param height The height of the extent.
    //  * @param object A geographic geometry collection supported by d3-geo (An extension of GeoJSON geometry collection).
    //  */
    // fitHeight(height: number, object: ExtendedGeometryCollection): this;

    // /**
    //  * Returns a new array [longitude, latitude] in degrees representing the unprojected point of the given projected point.
    //  * May return null if the specified point has no defined projected position, such as when the point is outside the clipPIng bounds of the projection.
    //  *
    //  * @param point The projected point, specified as a two-element array [x, y] (tyPIcally in PIxels).
    //  */
    // invert?(point: [number, number]): [number, number] | null;

    // /**
    //  * Returns the projection’s current resampling precision which defaults to square root of 0.5.
    //  * This value corresponds to the Douglas–Peucker distance.
    //  */
    fn get_precision(self) -> <Self as ProjectionTrait<'a>>::T;

    // /**
    //  * Sets the threshold for the projection’s adaptive resampling to the specified value in PIxels and returns the projection.
    //  * This value corresponds to the Douglas–Peucker distance.
    //  *
    //  * @param precision A numeric value in PIxels to use as the threshold for the projection’s adaptive resampling.
    //  */
    fn precision(
        self,
        delta: &'a <Self as ProjectionTrait<'a>>::T,
    ) -> Projection<Self::PR, <Self as ProjectionTrait<'a>>::T>
    where
        // Rc<<Self as ProjectionTrait<'a>>::PR>:
        //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        <Self as ProjectionTrait<'a>>::PR:
            Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        // <Self as ProjectionTrait<'a>>::SD:
        //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;

    fn get_reflect_x(&self) -> bool;

    fn reflect_x(self, reflect: bool) -> Projection<'a, Self::PR, <Self as ProjectionTrait<'a>>::T>
    where
        // Rc<<Self as ProjectionTrait<'a>>::PR>:
        //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        <Self as ProjectionTrait<'a>>::PR:
            Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        // <Self as ProjectionTrait<'a>>::SD:
        //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;

    fn get_reflect_y(&self) -> bool;

    fn reflect_y(self, reflect: bool) -> Projection<'a, Self::PR, <Self as ProjectionTrait<'a>>::T>
    where
        // Rc<<Self as ProjectionTrait<'a>>::PR>:
        //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        <Self as ProjectionTrait<'a>>::PR:
            Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        // <Self as ProjectionTrait<'a>>::SD:
        //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;

    // /**
    //  * Returns the projection’s current angle, which defaults to 0°.
    //  */
    // angle(): number;
    // /**
    //  * Sets the projection’s post-projection planar rotation angle to the specified angle in degrees and returns the projection.
    //  * @param angle The new rotation angle of the projection.
    //  */
    // angle(angle: number): this;

    fn reset(self) -> Projection<'a, Self::PR, <Self as ProjectionTrait<'a>>::T>
    where
        // Rc<<Self as ProjectionTrait<'a>>::PR>:
        //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        <Self as ProjectionTrait<'a>>::PR:
            Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        // <Self as ProjectionTrait<'a>>::SD:
        //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;

    fn recenter(self) -> Projection<'a, Self::PR, <Self as ProjectionTrait<'a>>::T>
    where
        // Rc<<Self as ProjectionTrait<'a>>::PR>:
        //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        <Self as ProjectionTrait<'a>>::PR:
            Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        // <Self as ProjectionTrait<'a>>::SD:
        //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;
    // /**
    //  * Sets the projection’s three-axis rotation to the specified angles, which must be a two- or three-element array of numbers.
    //  *
    //  * @param angles  A two- or three-element array of numbers [lambda, phi, gamma] specifying the rotation angles in degrees about each spherical axis.
    //  * (These correspond to yaw, PItch and roll.) If the rotation angle gamma is omitted, it defaults to 0.
    //  */
    fn get_rotate(&self) -> [<Self as ProjectionTrait<'a>>::T; 3]
    where
        <Self as ProjectionTrait<'a>>::PR:
            Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;

    fn rotate(
        self,
        angles: [<Self as ProjectionTrait<'a>>::T; 3],
    ) -> Projection<'a, Self::PR, <Self as ProjectionTrait<'a>>::T>
    where
        // Rc<<Self as ProjectionTrait<'a>>::PR>:
        //     Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        <Self as ProjectionTrait<'a>>::PR:
            Clone + Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        // <Self as ProjectionTrait<'a>>::SD:
        //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;

    fn stream(
        &self,
        stream_dst: Box<dyn Stream<SC = Coordinate<Self::T>>>,
    ) -> StreamTransformRadians<StreamTransform<Self::T>, Self::T>
    where
        <Self as ProjectionTrait<'a>>::PR:
            Transform<C = Coordinate<<Self as ProjectionTrait<'a>>::T>>,
        // SD: Stream<SC = Coordinate<Self::T>>,
        // <Self as ProjectionTrait<'a>>::SD:
        //     Stream<SC = Coordinate<<Self as ProjectionTrait<'a>>::T>> + Default,
        <Self as ProjectionTrait<'a>>::T: AddAssign
            + AsPrimitive<<Self as ProjectionTrait<'a>>::T>
            + Debug
            + Display
            + Float
            + FloatConst;
}
