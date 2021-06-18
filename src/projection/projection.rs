use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;
use std::rc::Rc;

// use derivative::Derivative;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::data_object::DataObject;
use crate::stream::Stream;
// use crate::stream::StreamSimpleNode;
use super::center::Center;
use super::clip_extent::ClipExtent;
use super::fit::fit_extent;
use crate::stream::stream_in_trait::StreamIn;
use crate::Transform;
// use super::resample::resample::Resample;
use super::resample::ResampleTrait;
use super::scale::Scale;
use super::scale_translate_rotate::ScaleTranslateRotate;
use super::scale_translate_rotate::ScaleTranslateRotateEnum;
use super::translate::Translate;
// use super::ProjectionRawTrait;
use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::circle::ClipCircle;
// use crate::clip::clip_sink_enum::ClipSinkEnum;
// use crate::clip::interpolate_trait::Interpolate;
// use crate::clip::point_visible_trait::PointVisible;
use crate::clip::Clip;
use crate::compose::Compose;
use crate::projection::projection_trait::ProjectionTrait;
use crate::projection::resample::gen_resample_node;
// use crate::projection::resample::ResampleEnum;
// use crate::projection::resample::Resample;
// use crate::projection::stream_transform::StreamTransform;
use crate::projection::stream_transform::StreamTransform;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::rotation::rotate_radians_enum::RotateRadiansEnum;
// use crate::rotation::rotate_radians_transform::rotate_radians_transform;
use crate::rotation::rotation_identity::RotationIdentity;
// use crate::stream::stream_dst::StreamDst;

pub enum StreamOrValueMaybe<T: CoordFloat> {
    Value(T),
    SP(Box<dyn Stream<SC = Coordinate<T>>>),
}

// #[derive(Derivative)]
// #[derivative(Debug)]
/// A collection of functions that mutate a Projection struct.
pub struct Projection<'a, PR, SD, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // SD: StreamDst,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
    SD: Stream<SC = Coordinate<T>> + Default,
    // dyn Clip<T=T>: Clip + Interpolate<T=T, C=Coordinate<T>> +Stream<T=T>,
    // T: stream:Stream:T,
{
    pd: PhantomData<&'a u8>,
    projection_raw: &'a PR,
    alpha: T, // post-rotate angle
    // cache: Option<
    //     Box<dyn Fn(Rc<RefCell<dyn Stream<C = Coordinate<T>>>>) -> StreamTransformRadiansNode<T>>,
    // >,
    // cache_stream: Option<StreamSimpleNode<T>>,
    // clip_antimeridian: Option<Box<dyn Transform<>>>,
    delta_lambda: T,
    delta_phi: T,
    delta_gamma: T,
    delta2: T, // precision
    k: T,      // scale

    project_resample: Box<dyn ResampleTrait<SInput = SD, SC = Coordinate<T>>>,
    project_transform: Compose<T, PR, ScaleTranslateRotateEnum<T>>,
    project_rotate_transform:
        Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,
    phi: T, // center
    // #[derivative(Debug = "ignore")]
    // preclip: Box<dyn Clip<SC = Coordinate<T>, PVC = Coordinate<T>>>,
    // #[derivative(Debug = "ignore")]
    // postclip: fn(ClipSinkEnum<PR, T>) -> ClipSinkEnum<PR, T>,
    postclip: fn(SD) -> SD,
    x: T,
    y: T, // translate
    lambda: T,
    rotate: RotateRadiansEnum<T>, //rotate, pre-rotate
    sx: T,                        // reflectX
    sy: T,                        // reflectY
    theta: Option<T>,
    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent
}

impl<'a, PR, SD, T> Projection<'a, PR, SD, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    SD: 'a + Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub fn new(projection_raw: &'a PR, delta2_p: Option<T>) -> Projection<'a, PR, SD, T> {
        let delta2 = match delta2_p {
            None => {
                T::from(0.5).unwrap() // precision
            }
            Some(delta2) => delta2,
        };
        // let projection_raw = Rc::new(projection_raw);

        let p = Self {
            pd: PhantomData,
            projection_raw,
            alpha: T::zero(), // post-rotate angle
            // cache: None,
            // cache_stream: None,
            delta2, // precision
            delta_lambda: T::zero(),
            delta_phi: T::zero(),
            delta_gamma: T::zero(),
            // scale
            k: T::from(150f64).unwrap(),
            // translate
            lambda: T::zero(),
            phi: T::zero(),
            rotate: RotateRadiansEnum::I(RotationIdentity::default()), // pre-rotate
            // preclip: Box::new(ClipAntimeridian::new(projection_raw)),
            postclip: |x| x,
            sx: T::one(), // reflectX
            sy: T::one(), // reflectX
            theta: None,  // pre-clip angle
            x: T::from(480f64).unwrap(),
            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent
            y: T::from(250).unwrap(),
            project_resample: gen_resample_node(projection_raw, T::zero()),
            project_transform: Compose::new(*projection_raw, ScaleTranslateRotateEnum::default()),
            project_rotate_transform: Compose::new(
                RotateRadiansEnum::I(RotationIdentity::default()),
                Compose::new(*projection_raw, ScaleTranslateRotateEnum::default()),
            ),
        };

        p.recenter()
    }
}
impl<'a, PR, SD, T> ClipExtent for Projection<'a, PR, SD, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    SD: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;
    type P = Projection<'a, PR, SD, T>;
    fn get_clip_extent(&self) -> Option<[Coordinate<T>; 2]> {
        match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
            }
            _ => None,
        }
    }

    fn clip_extent(mut self, extent: Option<[Coordinate<T>; 2]>) -> Projection<'a, PR, SD, T> {
        match extent {
            None => {
                self.x0 = None;
                self.y0 = None;
                self.x1 = None;
                self.y1 = None;
                // self.postclip = Identity;
                // Is this the identity projection Mutator???
                todo!("must implement identity");
            }
            Some(extent) => {
                // set x0 ...
                self.x0 = Some(extent[0].x);
                self.y0 = Some(extent[0].y);
                self.x1 = Some(extent[1].x);
                self.y1 = Some(extent[1].y);
                // todo!("must implement clip rectangle")
                // clipRectangle(self.x0, self.y0, self.x1, self.y1);
                self.reset()
            }
        }
    }
}

impl<'a, PR, SD, T> Center for Projection<'a, PR, SD, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    SD: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;
    type P = Projection<'a, PR, SD, T>;
    fn get_center(&self) -> Coordinate<T> {
        return Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        };
    }

    // TODO dynamic cast and unwrap - Must find a better way.
    fn center(self, p: Coordinate<T>) -> Projection<'a, PR, SD, T> {
        // self.lambda = (p[0] % F::from_u16(360u16).unwrap()).to_radians();
        // self.phi = (p[1] % F::from_u16(360u16).unwrap()).to_radians();
        self.recenter()
    }
}

impl<'a, PR, SD, T> ProjectionTrait<'a> for Projection<'a, PR, SD, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    SD: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;
    type PR = PR;
    type T = T;
    type SD = SD;
    // /**
    //  * Returns a new array [x, y] (tyPIcally in PIxels) representing the projected point of the given point.
    //  * The point must be specified as a two-element array [longitude, latitude] in degrees.
    //  * May return null if the specified point has no defined projected position, such as when the point is outside the clipPIng bounds of the projection.
    //  *
    //  * @param point A point specified as a two-dimensional array [longitude, latitude] in degrees.
    //  */
    // (point: [number, number]): [number, number] | null;

    // /**
    //  * Returns the current center of the projection, which defaults to ⟨0°,0°⟩.
    //  */
    // fn get_center(&self) -> Point;

    // /**
    //  * Sets the projection’s center to the specified center,
    //  * a two-element array of longitude and latitude in degrees and returns the projection.
    //  * The default is ⟨0°,0°⟩.
    //  *
    //  * @param point A point specified as a two-dimensional array [longitude, latitude] in degrees.
    //  */
    // fn center(&mut self, point: Point);

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
    fn clip_angle(mut self, angle: StreamOrValueMaybe<T>) -> Projection<'a, PR, SD, T> {
        match angle {
            StreamOrValueMaybe::Value(angle) => {
                let theta = angle.to_radians();
                self.theta = Some(theta);
                // self.preclip = Box::new(ClipCircle::new(self.projection_raw, theta));
                // println!("preclip {:#?}", self.preclip);
                // panic!("clip_angler stop");
            }
            StreamOrValueMaybe::SP(_preclip) => {
                todo!("must sort this out.");
                // self.theta = None;
                // self.preclip = preclip;
                // self.reset();
            }
        }
        self.reset()
    }

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
    #[inline]
    fn fit_extent(
        self,
        extent: [Coordinate<T>; 2],
        object: DataObject<T>,
    ) -> Projection<'a, PR, SD, T> {
        fit_extent(self, extent, object)
    }
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
    #[inline]
    fn reset(self) -> Projection<'a, PR, SD, T> {
        // self.cache_stream = None;
        // self.cache = None;
        self
    }

    fn recenter(self) -> Projection<'a, PR, SD, T> {
        let center = ScaleTranslateRotate::new(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coordinate {
            x: self.lambda,
            y: self.phi,
        }));

        let transform = ScaleTranslateRotate::new(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            self.alpha,
        );

        todo!("must refactor the stream pipeline");
        // self.rotate = rotate_radians_transform(self.delta_lambda, self.delta_phi, self.delta_gamma);
        // self.project_transform = Compose::new(self.projection_raw, transform);
        // self.project_rotate_transform = Compose::new(self.rotate, self.project_transform);

        // // self.project_resample = ResampleEnum::R(Resample::new(self.projection_raw));
        // self.project_resample.stream_in(self.project_transform);

        self.reset()
    }
    // /**
    //  * Returns the projection’s current resampling precision which defaults to square root of 0.5.
    //  * This value corresponds to the Douglas–Peucker distance.
    //  */
    // /**
    //  * Sets the threshold for the projection’s adaptive resampling to the specified value in PIxels and returns the projection.
    //  * This value corresponds to the Douglas–Peucker distance.
    //  *
    //  * @param precision A numeric value in PIxels to use as the threshold for the projection’s adaptive resampling.
    //  */
    #[inline]
    fn get_precision(self) -> T {
        self.delta2.sqrt()
    }

    #[inline]
    fn get_reflect_x(&self) -> bool {
        self.sx < T::zero()
    }

    fn reflect_x(mut self, reflect: bool) -> Projection<'a, PR, SD, T> {
        if reflect {
            self.sx = T::from(-1.0).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter()
    }

    #[inline]
    fn get_reflect_y(&self) -> bool {
        self.sy < T::zero()
    }

    #[inline]
    fn reflect_y(mut self, reflect: bool) -> Projection<'a, PR, SD, T> {
        if reflect {
            self.sy = T::from(-1.0).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter()
    }

    fn precision(mut self, delta: T) -> Projection<'a, PR, SD, T> {
        self.delta2 = delta * delta;
        self.project_resample = gen_resample_node(self.projection_raw, self.delta2);
        self.reset()
    }

    // /**
    //  * Returns the projection’s current angle, which defaults to 0°.
    //  */
    // angle(): number;
    // /**
    //  * Sets the projection’s post-projection planar rotation angle to the specified angle in degrees and returns the projection.
    //  * @param angle The new rotation angle of the projection.
    //  */
    // angle(angle: number): this;

    // /**
    //  * Sets the projection’s three-axis rotation to the specified angles, which must be a two- or three-element array of numbers.
    //  *
    //  * @param angles  A two- or three-element array of numbers [lambda, phi, gamma] specifying the rotation angles in degrees about each spherical axis.
    //  * (These correspond to yaw, PItch and roll.) If the rotation angle gamma is omitted, it defaults to 0.
    //  */
    #[inline]
    fn get_rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_lambda.to_degrees(),
        ]
    }

    fn rotate(mut self, angles: [T; 3]) -> Projection<'a, PR, SD, T> {
        let [delta_lambda, delta_phi, delta_gamma] = angles;
        let f360 = T::from(360f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter()
    }

    // fn get_clip_extent(&self) -> Option<[Coordinate<T>; 2]>;

    // fn clip_extent(self, extent: Option<[Coordinate<T>; 2]>) -> Projection<PR, T>;

    // fn get_scale(&self) -> T;
    // // /**
    // //  * Sets the projection’s scale factor to the specified value and returns the projection.
    // //  * The scale factor corresponds linearly to the distance between projected points; however, absolute scale factors are not equivalent across projections.
    // //  *
    // //  * @param scale Scale factor to be used for the projection; the default scale is projection-specific.
    // //  */
    // // fn scale(&mut self, scale: &F);
    // fn scale(self, scale: T) -> Projection<PR, T>;

    // In javascript stream is used as a property to be removed from the object.
    // In rust that is a closure.
    fn stream(&self, stream_dst: SD) -> StreamTransformRadians<SD, T>
    where
        SD: Stream<SC = Coordinate<T>> + Default,
    {
        // return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
        // return match &self.cache {
        //     Some(c) => Box::new(*c),
        //     None => {
        // self.cache_stream = Some(stream.clone());

        // let mut postclip = self.postclip.clone();
        // postclip.stream_in(ClipSinkEnum::Src(stream_dst));
        let postclip = (self.postclip)(stream_dst);

        let mut resample = self.project_resample;
        resample.stream_in(postclip);
        // let mut preclip = self.preclip;
        // preclip.stream_in(resample);

        // using resample here bypasses preclip.
        let mut t_rotate_node = StreamTransform::new(Some(self.rotate), resample);

        let mut t_radians_node = StreamTransformRadians::new(t_rotate_node);
        // t_radians_node.stream_in(t_rotate_node);

        // Output.
        t_radians_node

        //     }
        // };
    }

    // fn get_translate(&self) -> Coordinate<T>;

    // // /**
    // //  * Sets the projection’s translation offset to the specified two-element array [tx, ty] and returns the projection.
    // //  * The translation offset determines the PIxel coordinates of the projection’s center. The default translation offset places ⟨0°,0°⟩ at the center of a 960×500 area.
    // //  *
    // //  * @param point A two-element array [tx, ty] specifying the translation offset. The default translation offset of defaults to [480, 250] places ⟨0°,0°⟩ at the center of a 960×500 area.
    // //  */
    // fn translate(self, t: &Coordinate<T>) -> Projection<PR, T>;
}

impl<'a, PR, SD, T> Scale for Projection<'a, PR, SD, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    PR: Transform<C = Coordinate<T>>,
    SD: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type P = Projection<'a, PR, SD, T>;
    type T = T;
    #[inline]
    fn get_scale(&self) -> T {
        self.k
    }

    fn scale(mut self, scale: T) -> Projection<'a, PR, SD, T> {
        self.k = scale;
        self.recenter()
    }
}

impl<'a, PR, SD, T> Transform for Projection<'a, PR, SD, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    PR: Transform<C = Coordinate<T>>,
    SD: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let r = Coordinate {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        self.project_rotate_transform.transform(&r)
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let d = self.project_rotate_transform.invert(p);
        Coordinate {
            x: d.x.to_degrees(),
            y: d.y.to_degrees(),
        }
    }
}

impl<'a, PR, SD, T> Translate for Projection<'a, PR, SD, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    PR: Transform<C = Coordinate<T>>,
    SD: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type P = Projection<'a, PR, SD, T>;
    type C = Coordinate<T>;
    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> Projection<'a, PR, SD, T> {
        self.x = t.x;
        self.y = t.y;
        self.recenter()
    }
}
