use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::circle::gen_clip_factory_circle;
use crate::clip::circle::line::Line as CircleLine;
use crate::clip::circle::pv::PV as CirclePV;
use crate::clip::rectangle::rectangle::Rectangle as ClipRectangle;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::compose::Compose;
use crate::identity::Identity;
use crate::projection::post_clip_node::PostClipNode;
use crate::projection::Precision;
use crate::rotation::rotate_radians;
use crate::rotation::rotate_radians::RotateRadians;
use crate::stream::Stream;
use crate::Transform;

use super::fit::fit_extent;
use super::fit::fit_size;
use super::post_clip::PostClip;
use super::resample::stream_node_resample_factory::StreamNodeResampleFactory;
use super::resample::ResampleNode;
use super::str::generate as generate_str;
use super::str::scale_translate_rotate::ScaleTranslateRotate;
use super::stream_node_factory::StreamNodeFactory;
use super::stream_node_post_clip_factory::StreamNodePostClipFactory;
use super::stream_transform_radians::StreamTransformRadians;
use super::Angle;
use super::BoundsStream;
use super::Center;
use super::ClipExtent;
use super::DataObject;
use super::Fit;
use super::Projection;
use super::Raw as ProjectionRaw;
use super::Reflect;
use super::Rotate;
use super::RotateFactory;
use super::RotateTransformFactory;
use super::Scale;
use super::Translate;

/// Projection builder.
///
/// Holds State related to the construction of the a projection.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T> + Transform<T = T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
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

    /// Used by recenter() to build the factories.
    pub rotate: RotateRadians<T>, //rotate, pre-rotate
    project_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
    /// Used by rotate_transform_factory and projections transform.
    pub project_rotate_transform:
        Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    /// Projection pipeline stage.
    pub postclip_factory: StreamNodePostClipFactory<DRAIN, T>,
    /// Projection pipeline stage.
    pub preclip_factory:
        StreamNodeClipFactory<L, PR, PV, ResampleNode<PR, PostClipNode<DRAIN, T>, T>, T>,
    /// Projection pipeline stage.
    pub rotate_factory: RotateFactory<DRAIN, L, PR, PV, T>,
    /// Projection pipeline stage
    pub resample_factory: StreamNodeResampleFactory<PR, PostClipNode<DRAIN, T>, T>,
    /// Projection pipeline stage
    pub rotate_transform_factory: RotateTransformFactory<DRAIN, L, PR, PV, T>,
}

impl<DRAIN, L, PR, PV, T> Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    pub fn new(
        preclip_factory: StreamNodeClipFactory<
            L,
            PR,
            PV,
            ResampleNode<PR, PostClipNode<DRAIN, T>, T>,
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

        let center = generate_str(&k, &T::zero(), &T::zero(), &sx, &sy, &alpha)
            .transform(&projection_raw.transform(&Coordinate { x: lambda, y: phi }));
        let str = generate_str(&k, &(x - center.x), &(y - center.y), &sx, &sy, &alpha);

        let rotate = rotate_radians([delta_lambda, delta_phi, delta_gamma]); // pre-rotate
        let project_transform = Compose::new(projection_raw.clone(), str);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());

        let postclip_factory = StreamNodePostClipFactory::new(PostClip::I(Identity {}));
        let rotate_factory = StreamNodeFactory::new(rotate.clone());
        let resample_factory = StreamNodeResampleFactory::new(project_transform.clone(), T::zero());
        let rotate_transform_factory = StreamNodeFactory::new(project_rotate_transform.clone());

        let out = Self {
            /// Input passing onto Projection.
            projection_raw,

            /// Internal state
            delta_lambda,
            delta_phi,
            delta_gamma,
            // postclip: Rc::new(|x| x),
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

            rotate,
            project_transform,
            project_rotate_transform,
            /// Pass into Projection,
            postclip_factory,
            preclip_factory,
            resample_factory,
            rotate_factory,
            rotate_transform_factory,
        };

        out.recenter()
    }

    /// Using the currently programmed state output a new projection.
    #[inline]
    pub fn build(&self) -> Projection<DRAIN, L, PR, PV, T> {
        Projection {
            postclip_factory: self.postclip_factory.clone(),
            preclip_factory: self.preclip_factory.clone(),
            resample_factory: self.resample_factory.clone(),

            rotate_transform: self.project_rotate_transform.clone(),
            rotate_transform_factory: self.rotate_transform_factory.clone(),
            rotate_factory: self.rotate_factory.clone(),
            transform_radians_factory: StreamNodeFactory::new(StreamTransformRadians {}),
        }
    }

    /// Switches to antimeridian cutting rather than small-circle clipPIng.
    /// See also projection.preclip, d3.geoClipAntimeridian, d3.geoClipCircle.
    ///
    /// @param angle Set to null to switch to antimeridian cutting.
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
        let center = generate_str(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            &self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coordinate {
            x: self.lambda,
            y: self.phi,
        }));
        let transform = generate_str(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            &self.alpha,
        );

        self.rotate = rotate_radians([self.delta_lambda, self.delta_phi, self.delta_gamma]);
        self.project_transform = Compose::new(self.projection_raw.clone(), transform);
        self.project_rotate_transform =
            Compose::new(self.rotate.clone(), self.project_transform.clone());

        self.resample_factory =
            StreamNodeResampleFactory::new(self.project_transform.clone(), self.delta2);
        self.rotate_factory = StreamNodeFactory::new(self.rotate.clone());
        self.rotate_transform_factory =
            StreamNodeFactory::new(self.project_rotate_transform.clone());

        self.reset()
    }
}

impl<DRAIN, L, PR, PV, T> Translate for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> Self {
        self.x = t.x;
        self.y = t.y;
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> Center for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        }
    }

    fn center(mut self, p: Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter()
    }
}

impl<L, PR, PV, T> Fit for Builder<BoundsStream<T>, L, PR, PV, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn fit_extent(self, extent: [[T; 2]; 2], object: DataObject<Self::T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        fit_extent(self, extent, object)
    }

    #[inline]
    fn fit_size(self, size: [T; 2], object: DataObject<T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        fit_size(self, size, object)
    }
}

impl<DRAIN, L, PR, PV, T> Angle for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    /// f64 or f32
    type T = T;

    /// Returns the projection’s post-projection planar rotation angle.
    /// defaults to 0°.
    #[inline]
    fn get_angle(&self) -> Self::T {
        self.alpha.to_degrees()
    }

    /// Sets the projection’s post-projection planar rotation angle to the
    /// specified angle in degrees and returns the projection.
    ///
    fn angle(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360).unwrap()).to_radians();
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> Scale for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_scale(&self) -> Self::T {
        self.k
    }

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> ClipExtent for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    fn get_clip_extent(&self) -> Option<[Coordinate<T>; 2]> {
        match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
            }
            _ => None,
        }
    }

    fn clip_extent(mut self, extent: Option<[Coordinate<T>; 2]>) -> Self {
        match extent {
            None => {
                self.x0 = None;
                self.y0 = None;
                self.x1 = None;
                self.y1 = None;
                self.postclip_factory = StreamNodePostClipFactory::new(PostClip::I(Identity {}));
                self
            }
            Some(extent) => {
                // set x0 ...
                self.x0 = Some(extent[0].x);
                self.y0 = Some(extent[0].y);
                self.x1 = Some(extent[1].x);
                self.y1 = Some(extent[1].y);
                self.postclip_factory =
                    StreamNodePostClipFactory::new(PostClip::R(ClipRectangle::new(
                        self.x0.unwrap(),
                        self.y0.unwrap(),
                        self.x1.unwrap(),
                        self.y1.unwrap(),
                    )));
                self.reset()
            }
        }
    }
}

impl<DRAIN, L, PR, PV, T> Precision for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    /// /**
    ///  * Returns the projection’s current resampling precision which defaults to square root of 0.5.
    ///  * This value corresponds to the Douglas–Peucker distance.
    ///  */
    /// /**
    ///  * Sets the threshold for the projection’s adaptive resampling to the specified value in PIxels and returns the projection.
    ///  * This value corresponds to the Douglas–Peucker distance.
    ///  *
    ///  * @param precision A numeric value in PIxels to use as the threshold for the projection’s adaptive resampling.
    ///  */
    #[inline]
    fn get_precision(&self) -> T {
        self.delta2.sqrt()
    }

    /// Set the projection builder precision
    ///
    /// delta is related to clip angle.
    fn precision(mut self, delta: &T) -> Self {
        let delta2 = *delta * *delta;
        self.delta2 = *delta * *delta;
        self.resample_factory =
            StreamNodeResampleFactory::new(self.project_transform.clone(), delta2);
        self
    }
}

impl<DRAIN, L, PR, PV, T> Rotate for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    ///  Sets the projection’s three-axis rotation to the specified angles, which must be a two- or three-element array of numbers.
    ///
    ///  @param angles  A two- or three-element array of numbers [lambda, phi, gamma] specifying the rotation angles in degrees about each spherical axis.
    ///  (These correspond to yaw, PItch and roll.) If the rotation angle gamma is omitted, it defaults to 0.
    ///
    #[inline]
    fn get_rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_lambda.to_degrees(),
        ]
    }

    /// Sets the rotation angles as measured in degrees.
    fn rotate(mut self, angles: [T; 3]) -> Builder<DRAIN, L, PR, PV, T> {
        let [delta_lambda, delta_phi, delta_gamma] = angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> Reflect for Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T>,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    /// Is the projection builder set to invert the x-coordinate.
    #[inline]
    fn get_reflect_x(&self) -> bool {
        self.sx < T::zero()
    }

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x(mut self, reflect: bool) -> Self {
        if reflect {
            self.sx = T::from(-1.0).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter()
    }

    /// Is the projection builder set to invert the y-coordinate.
    #[inline]
    fn get_reflect_y(&self) -> bool {
        self.sy < T::zero()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y(mut self, reflect: bool) -> Self {
        if reflect {
            self.sy = T::from(-1.0).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter()
    }
}

impl<DRAIN, L, PR, PV, T> Builder<DRAIN, L, PR, PV, T>
where
    DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
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
}
