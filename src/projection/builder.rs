use crate::clip::clip::Clip;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Projection;
use crate::projection::RefCell;
use crate::projection::StreamNode;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::circle::gen_clip_factory_circle;
use crate::clip::circle::line::Line as CircleLine;
use crate::clip::circle::pv::PV as CirclePV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::clip::LineRaw;
use crate::clip::PointVisible;
use crate::compose::Compose;
use crate::rotation::rotate_radians::rotate_radians;
use crate::rotation::rotate_radians_enum::RotateRadiansEnum;
use crate::stream::Stream;
use crate::Transform;

use super::center::Center;
use super::clip_extent::ClipExtent;
use super::resample::gen_resample_factory;
use super::resample::ResampleEnum;
use super::scale::Scale;
use super::scale_translate_rotate::ScaleTranslateRotate;
use super::scale_translate_rotate::ScaleTranslateRotateEnum;
use super::stream_node_factory::StreamNodeFactory;
use super::translate::Translate;
use super::Raw as ProjectionRaw;

#[derive(Clone)]
pub struct Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    projection_raw: PR,

    phi: T, // center
    lambda: T,

    alpha: T, // post-rotate angle
    k: T,     // scale
    sx: T,    // reflectX
    sy: T,    // reflectY

    x: T,
    y: T, // translate

    delta_lambda: T,
    delta_phi: T,
    delta_gamma: T,

    delta2: T, // precision

    theta: Option<T>,

    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent
    preclip_factory: StreamNodeClipFactory<L, PR, PV, StreamNode<ResampleEnum<PR, T>, DRAIN, T>, T>,

    postclip: Rc<dyn Fn(Rc<RefCell<DRAIN>>) -> Rc<RefCell<DRAIN>>>,
    // Used by recenter() to build the factories.
    rotate: RotateRadiansEnum<T>, //rotate, pre-rotate
    transform: Compose<T, PR, ScaleTranslateRotateEnum<T>>,
    rotate_transform: Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,

    resample_factory: StreamNodeFactory<ResampleEnum<PR, T>, DRAIN, T>,
    rotate_transform_factory: StreamNodeFactory<
        Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,
        StreamNode<
            Clip<L, PV, StreamNode<ResampleEnum<PR, T>, DRAIN, T>, T>,
            StreamNode<ResampleEnum<PR, T>, DRAIN, T>,
            T,
        >,
        T,
    >,
}

impl<DRAIN, L, PR, PV, T> Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T> + Clone + Copy,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(
        preclip_factory: StreamNodeClipFactory<
            L,
            PR,
            PV,
            StreamNode<ResampleEnum<PR, T>, DRAIN, T>,
            T,
        >,
        projection_raw: PR,
    ) -> Self {
        let x = T::from(480_f64).unwrap();
        let y = T::from(250_f64).unwrap();
        let lambda = T::zero();
        let phi = T::zero();
        let alpha = T::zero();
        let k = T::from(150_f64).unwrap();
        let sx = T::one();
        let sy = T::one();
        let delta_lambda = T::zero();
        let delta_phi = T::zero();
        let delta_gamma = T::zero();

        // Zero implies no resampling by default.
        let resample_factory = gen_resample_factory(projection_raw, T::zero());

        let center = ScaleTranslateRotate::new(&k, &T::zero(), &T::zero(), &sx, &sy, alpha)
            .transform(&projection_raw.transform(&Coordinate { x: lambda, y: phi }));
        let str = ScaleTranslateRotate::new(&k, &(x - center.x), &(y - center.y), &sx, &sy, alpha);

        let rotate = rotate_radians(delta_lambda, delta_phi, delta_gamma); // pre-rotate
        let transform = Compose::new(projection_raw, str);
        let rotate_transform = Compose::new(rotate.clone(), transform.clone());
        let rotate_transform_factory = StreamNodeFactory::new(rotate_transform.clone());

        Self {
            /// Input passing onto Projection.
            projection_raw,

            /// Internal state
            delta_lambda,
            delta_phi,
            delta_gamma,
            postclip: Rc::new(|x| x),
            x,
            y,

            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent

            delta2: T::from(0.5_f64).unwrap(),
            lambda,
            phi,

            alpha,
            k,
            theta: None,
            sx,
            sy,

            preclip_factory,
            resample_factory,
            transform,
            rotate_transform,
            /// Pass into Projection,
            rotate,
            rotate_transform_factory,
        }
    }

    fn build(&self) -> Projection<DRAIN, L, PR, PV, T> {
        let transform_radians_factory: StreamNodeFactory<
            StreamTransformRadians,
            StreamNode<
                Compose<T, RotateRadiansEnum<T>, Compose<T, PR, ScaleTranslateRotateEnum<T>>>,
                StreamNode<
                    Clip<L, PV, StreamNode<ResampleEnum<PR, T>, DRAIN, T>, T>,
                    StreamNode<ResampleEnum<PR, T>, DRAIN, T>,
                    T,
                >,
                T,
            >,
            T,
        >;

        transform_radians_factory = StreamNodeFactory::new(StreamTransformRadians {});
        Projection {
            postclip: self.postclip.clone(),
            preclip_factory: self.preclip_factory.clone(),
            projection_raw: self.projection_raw,

            resample_factory: self.resample_factory.clone(),

            rotate: self.rotate.clone(),

            rotate_transform: self.rotate_transform.clone(),
            rotate_transform_factory: self.rotate_transform_factory.clone(),
            transform_radians_factory,
        }
    }

    // /**
    //  * Switches to antimeridian cutting rather than small-circle clipPIng.
    //  * See also projection.preclip, d3.geoClipAntimeridian, d3.geoClipCircle.
    //  *
    //  * @param angle Set to null to switch to antimeridian cutting.
    //  */
    pub fn clip_angle(mut self, angle: T) -> Builder<DRAIN, CircleLine<T>, PR, CirclePV<T>, T> {
        self.theta = Some(angle.to_radians());

        // match angle {
        //     StreamOrValueMaybe::Value(angle) => {
        //         let theta = angle.to_radians();
        //         self.theta = Some(theta);
        //         // self.preclip = Box::new(ClipCircle::new(self.projection_raw, theta));
        //         // println!("preclip {:#?}", self.preclip);
        //         // panic!("clip_angler stop");
        //     }
        //     StreamOrValueMaybe::SP(_preclip) => {
        //         todo!("must sort this out.");
        //         // self.theta = None;
        //         // self.preclip = preclip;
        //         // self.reset();
        //     }
        // }

        // Only change is the resample_factory.
        let preclip_factory = gen_clip_factory_circle(angle);
        let mut out = Builder::new(preclip_factory, self.projection_raw);
        out.theta = Some(angle.to_radians());
        out
    }

    fn reset(self) -> Builder<DRAIN, L, PR, PV, T> {
        // self.cache_stream = None;
        // self.cache = None;
        self
    }

    fn recenter(mut self) -> Builder<DRAIN, L, PR, PV, T> {
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

        let str = ScaleTranslateRotate::new(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            self.alpha,
        );

        self.rotate = rotate_radians(self.delta_lambda, self.delta_phi, self.delta_gamma);
        self.transform = Compose::new(self.projection_raw.clone(), str);
        self.rotate_transform = Compose::new(self.rotate.clone(), self.transform.clone());

        //todo update every factory.
        self.resample_factory = gen_resample_factory(self.projection_raw, self.delta2);

        self.reset()
    }
}

impl<DRAIN, L, PR, PV, T> Translate for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type P = Builder<DRAIN, L, PR, PV, T>;
    type C = Coordinate<T>;
    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> Builder<DRAIN, L, PR, PV, T> {
        self.x = t.x;
        self.y = t.y;
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> Center for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type C = Coordinate<T>;

    fn get_center(&self) -> Coordinate<T> {
        return Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        };
    }

    fn center(mut self, p: Coordinate<T>) -> Builder<DRAIN, L, PR, PV, T> {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> Scale for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T> + Clone + Copy,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type ST = T;
    #[inline]
    fn get_scale(&self) -> Self::ST {
        self.k
    }

    fn scale(mut self, scale: T) -> Builder<DRAIN, L, PR, PV, T> {
        self.k = scale;
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> ClipExtent for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn get_clip_extent(&self) -> Option<[Coordinate<T>; 2]> {
        match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
            }
            _ => None,
        }
    }

    fn clip_extent(mut self, extent: Option<[Coordinate<T>; 2]>) -> Builder<DRAIN, L, PR, PV, T> {
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

impl<DRAIN, L, PR, PV, T> Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<SC = Coordinate<T>>,
    L: LineRaw,
    PR: ProjectionRaw<T = T>,
    PV: PointVisible<T = T>,

    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    // type C = Coordinate<T>;
    // type PR = PR;
    // type T = T;
    // type DRAIN = DRAIN;
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
    // #[inline]

    // fn fit_extent(
    //     self,
    //     extent: [Coordinate<T>; 2],
    //     object: DataObject<T>,
    // ) -> Projection<'a, PR, SD, T> {
    //     fit_extent(self, extent, object)
    // }

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
    pub fn get_precision(self) -> T {
        self.delta2.sqrt()
    }

    #[inline]
    pub fn get_reflect_x(&self) -> bool {
        self.sx < T::zero()
    }

    pub fn reflect_x(mut self, reflect: bool) -> Self {
        if reflect {
            self.sx = T::from(-1.0).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter()
    }

    #[inline]
    pub fn get_reflect_y(&self) -> bool {
        self.sy < T::zero()
    }

    #[inline]
    pub fn reflect_y(mut self, reflect: bool) -> Self {
        if reflect {
            self.sy = T::from(-1.0).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter()
    }

    pub fn precision(self, delta: &T) -> Builder<DRAIN, L, PR, PV, T> {
        let mut out = Builder::new(self.preclip_factory, self.projection_raw.clone());
        out.resample_factory = gen_resample_factory(self.projection_raw, self.delta2);
        out.delta2 = *delta * *delta;
        out
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
    pub fn get_rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_lambda.to_degrees(),
        ]
    }

    pub fn rotate(mut self, angles: [T; 3]) -> Builder<DRAIN, L, PR, PV, T> {
        let [delta_lambda, delta_phi, delta_gamma] = angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter()
    }
}
