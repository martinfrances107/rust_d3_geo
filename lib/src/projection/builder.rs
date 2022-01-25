use approx::AbsDiffEq;
use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::circle::gen_clip_factory_circle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::clip::post_clip::PostClip;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::rectangle::Rectangle as ClipRectangle;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::clip::stream_node_post_clip_factory::StreamNodePostClipFactory;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::compose::Compose;
use crate::identity::Identity;
use crate::projection::Precision;
use crate::rotation::rotate_radians;
use crate::rotation::rotate_radians::RotateRadians;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::fit::fit_extent;
use super::fit::fit_height;
use super::fit::fit_size;
use super::fit::fit_width;
use super::resample::stream_node_resample_factory::StreamNodeResampleFactory;
use super::resample::ResampleNode;
use super::str::generate as generate_str;
use super::str::scale_translate_rotate::ScaleTranslateRotate;
use super::stream_node::StreamNode;
use super::stream_node_factory::StreamNodeFactory;
use super::stream_transform_radians::StreamTransformRadians;
use super::Angle;
use super::Bounds;
use super::Center;
use super::ClipAngle;
use super::ClipExtent;
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
pub struct Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T> + Transform<T = T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
    pub preclip_factory: StreamNodeClipFactory<
        DRAIN,
        LINE,
        PR,
        PV,
        ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>,
        T,
    >,
    /// Projection pipeline stage.
    pub rotate_factory: RotateFactory<DRAIN, DRAIN, LINE, PR, PV, T>,

    /// Projection pipeline stage
    pub resample_factory: StreamNodeResampleFactory<PR, PostClipNode<DRAIN, DRAIN, T>, T>,
    /// Projection pipeline stage
    pub rotate_transform_factory: RotateTransformFactory<DRAIN, DRAIN, LINE, PR, PV, T>,
}

impl<DRAIN, LINE, PR, PV, T> Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    pub fn new(
        preclip_factory: StreamNodeClipFactory<
            DRAIN,
            LINE,
            PR,
            PV,
            ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>,
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

            /// Internal state.
            delta_lambda,
            delta_phi,
            delta_gamma,
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
    pub fn build(&self) -> Projection<DRAIN, LINE, PR, PV, T> {
        Projection {
            cache: None,
            postclip_factory: self.postclip_factory.clone(),
            preclip_factory: self.preclip_factory.clone(),
            resample_factory: self.resample_factory.clone(),

            rotate_transform: self.project_rotate_transform.clone(),
            rotate_transform_factory: self.rotate_transform_factory.clone(),
            rotate_factory: self.rotate_factory.clone(),
            transform_radians_factory: StreamNodeFactory::new(StreamTransformRadians {}),
        }
    }

    fn reset(self) -> Self {
        // self.cache_stream = None;
        // self.cache = None;
        self
    }

    fn recenter(mut self) -> Self {
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

impl<DRAIN, LINE, PR, PV, T> ClipAngle for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type Drain = DRAIN;
    type PR = PR;

    // Set the internal clip angle (theta) to null and return a builder
    // which uses the antimeridian clipping stratergy.
    // fn clip_angle_reset(self) -> Builder<DRAIN, LineAntimeridian<T>, PR, PVAntimeridian<T>, T> {
    //     let preclip_factory: StreamNodeClipFactory<
    //         DRAIN,
    //         LineAntimeridian<T>,
    //         PR,
    //         PVAntimeridian<T>,
    //         DRAIN,
    //         T,
    //     > = gen_clip_factory_antimeridian();
    //     // let preclip_factory = gen_clip_factory_antimeridian();

    //     let out = Builder::new(preclip_factory, self.projection_raw);
    //     // update only theta and preclip_factory.
    //     // let out: Builder<DRAIN, LineAntimeridian<T>, PR, PVAntimeridian<T>, T> = Builder {
    //     //     // projection_raw: self.projection_raw,
    //     //     /// Internal state.
    //     //     // delta_lambda: self.delta_lambda,
    //     //     // delta_phi: self.delta_phi,
    //     //     // delta_gamma: self.delta_gamma,

    //     //     // x: self.x,
    //     //     // y: self.y,

    //     //     // x0: self.x0,
    //     //     // y0: self.y0,
    //     //     // x1: self.x1,
    //     //     // y1: self.y1,
    //     //     // delta2: self.delta2,
    //     //     // lambda: self.lambda,
    //     //     // phi: self.phi,

    //     //     // alpha: self.alpha,
    //     //     // k: self.k,
    //     //     // theta: None,
    //     //     // sx: self.sx,
    //     //     // sy: self.sy,
    //     //     // rotate: self.rotate.clone(),
    //     //     // project_transform: self.project_transform,
    //     //     // project_rotate_transform: self.project_rotate_transform.clone(),
    //     //     // postclip_factory: self.postclip_factory,
    //     //     preclip_factory,
    //     //     // resample_factory: self.resample_factory,

    //     //     // rotate_transform_factory: StreamNodeFactory::new(self.project_rotate_transform),
    //     //     // rotate_factory: StreamNodeFactory::new(self.rotate),
    //     // };
    //     out
    //     // out.reset()
    // }

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    fn clip_angle(self, angle: T) -> Builder<DRAIN, LineCircle<T>, PR, PVCircle<T>, T> {
        if angle == T::zero() {
            panic!("must call clip_angle_reset() instead");
        }

        let theta = angle.to_radians();

        let cf: StreamNodeClipFactory<DRAIN, LineCircle<T>, PR, PVCircle<T>, _, T> =
            gen_clip_factory_circle(theta);
        let out: Builder<DRAIN, LineCircle<T>, PR, PVCircle<T>, T> =
            Builder::<DRAIN, LineCircle<T>, PR, PVCircle<T>, T>::new(cf, self.projection_raw);

        // Update only theta and preclip_factory.
        Builder::<DRAIN, LineCircle<T>, PR, PVCircle<T>, T> {
            // /// Internal state.
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            x: self.x,
            y: self.y,

            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,

            delta2: self.delta2,
            lambda: self.lambda,
            phi: self.phi,

            alpha: self.alpha,
            k: self.k,

            theta: Some(theta),

            sx: self.sx,
            sy: self.sy,

            rotate: self.rotate.clone(),
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
            postclip_factory: self.postclip_factory,

            resample_factory: self.resample_factory,
            // TODO do I need this line.
            // rotate_transform_factory: StreamNodeFactory::new(self.project_rotate_transform),
            rotate_factory: StreamNodeFactory::new(self.rotate),
            ..out
        }
    }
}

impl<DRAIN, LINE, PR, PV, T> Translate for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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

impl<DRAIN, LINE, PR, PV, T> Center for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        }
    }

    fn center(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter()
    }
}

impl<LINE, PR, PV, T> Fit for Builder<Bounds<T>, LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        Bounds<T>,
        LINE,
        ResampleNode<Bounds<T>, PR, PostClipNode<Bounds<T>, Bounds<T>, T>, T>,
        T,
    >: Stream<EP = Bounds<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Sets the projection’s scale and translate to fit the specified GeoJSON
    /// object in the center of the given extent.
    ///
    /// The extent is specified as an array [[x₀, y₀], [x₁, y₁]],
    /// where x₀ is the left side of the bounding box, y₀ is the top, x₁ is
    /// the right and y₁ is the bottom. Returns the projection.
    ///
    /// For example, to scale and translate the New Jersey State Plane
    /// projection to fit a GeoJSON object nj in the center of a 960×500
    /// bounding box with 20 pixels of padding on each side:
    #[inline]
    fn fit_extent(self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        fit_extent(self, extent, object)
    }

    /// Similar to fitExtent where the top-left corner of the extent is [0, 0].
    #[inline]
    fn fit_size(self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        fit_size(self, size, object)
    }

    /// Similar to fit_size where the height is automatically chosen from
    /// the aspect ratio of object and the given constraint on width.
    #[inline]
    fn fit_width(self, w: T, object: &impl Streamable<T = T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        fit_width(self, w, object)
    }

    /// Similar to fit_size where the width is automatically chosen from
    /// the aspect ratio of object and the given constraint on height.
    #[inline]
    fn fit_height(self, h: T, object: &impl Streamable<T = T>) -> Self
    where
        Self::T: AsPrimitive<T> + CoordFloat,
    {
        fit_height(self, h, object)
    }
}

impl<DRAIN, LINE, PR, PV, T> Angle for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,

    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// f64 or f32.
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

impl<DRAIN, LINE, PR, PV, T> Scale for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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

impl<DRAIN, LINE, PR, PV, T> ClipExtent for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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

    fn clip_extent_clear(mut self) -> Self {
        self.x0 = None;
        self.y0 = None;
        self.x1 = None;
        self.y1 = None;
        self.postclip_factory = StreamNodePostClipFactory::new(PostClip::I(Identity {}));
        self
    }

    fn clip_extent(mut self, extent: &[Coordinate<T>; 2]) -> Self {
        // set x0 ...
        self.x0 = Some(extent[0].x);
        self.y0 = Some(extent[0].y);
        self.x1 = Some(extent[1].x);
        self.y1 = Some(extent[1].y);
        self.postclip_factory = StreamNodePostClipFactory::new(PostClip::R(ClipRectangle::new(
            self.x0.unwrap(),
            self.y0.unwrap(),
            self.x1.unwrap(),
            self.y1.unwrap(),
        )));
        self.reset()
    }
}

impl<DRAIN, LINE, PR, PV, T> Precision for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
        self.delta2 = *delta * *delta;
        self.resample_factory =
            StreamNodeResampleFactory::new(self.project_transform.clone(), self.delta2);
        self
    }
}

impl<DRAIN, LINE, PR, PV, T> Rotate for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
    fn rotate(mut self, angles: &[T; 3]) -> Self {
        let [delta_lambda, delta_phi, delta_gamma] = *angles;
        let f360 = T::from(360_f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter()
    }
}

impl<DRAIN, LINE, PR, PV, T> Reflect for Builder<DRAIN, LINE, PR, PV, T>
where
    DRAIN: Stream<EP = DRAIN, T = T>,
    LINE: Line,
    StreamNode<DRAIN, LINE, ResampleNode<DRAIN, PR, PostClipNode<DRAIN, DRAIN, T>, T>, T>:
        Stream<EP = DRAIN, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
