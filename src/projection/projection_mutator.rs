use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// A collection of functions that mutate a Projection struct.

use super::projection::Projection;
use super::projection::StreamOrValueMaybe;
use crate::compose::Compose;
use crate::compose::ComposeElemEnum;
use crate::projection::resample::gen_resample_node;
use crate::projection::resample::ResampleEnum;
// use crate::projection::resample::ResampleNone;

// use crate::projection::stream_transform::StreamPreclipIn;
use crate::projection::stream_transform::StreamTransform;
use crate::projection::stream_transform_radians::StreamTransformIn;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::rotation::rotation_identity::RotationIdentity;
// use crate::projection::stream_transform_radians::StreamTransformRadiansNode;
// use crate::projection::stream_transform_radians::StreamTransformRadiansNodeStub;
use super::orthographic::OrthographicRaw;
use crate::clip::ClipSinkEnum;
use crate::rotation::rotate_radians_transform::rotate_radians_transform;
use crate::rotation::rotate_radians_transform::RotateRadiansEnum;
// use crate::stream::CompareIntersection;
// use crate::stream::stream_pipe::StreamPipe;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::stream::StreamDst;
// use crate::stream::StreamPostClipNode;
// use crate::stream::stream_postclip_node_stub::StreamPostClipNodeStub;
// use crate::stream::StreamPostClipTrait;
// use crate::stream::StreamPreClipNode;
use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::circle::ClipCircle;
// use crate::stream::stream_preclip_node_stub::StreamPreClipNodeStub;
// use crate::stream::StreamPreClipTrait;
// use crate::stream::StreamResampleNode;
// use crate::stream::StreamResampleTrait;
use super::resample::StreamResampleTrait;
use super::ProjectionRawEnum;
// use crate::stream::StreamSimpleNode;
// use crate::clip::ClipRaw;

use crate::clip::clip::Clip;
use crate::Transform;
use crate::TransformClone;
use crate::TransformIdentity;
// clip generators.
// use crate::clip::antimeridian::generate_antimeridian;

use super::scale_translate_rotate::ScaleTranslateRotate;
// use super::path::PathTrait;
use super::resample::resample::Resample;
// use super::resample::resample_none::ResampleNone;
// use crate::stream::StreamSourceDummy;

#[derive(Clone, Debug)]
pub struct ProjectionMutator<T: CoordFloat + FloatConst + Default> {
    // The mutator lives as long a the proejction it contnains.
    project: ProjectionRawEnum<T>,
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

    project_resample: ResampleEnum<T>,
    // project_transform: Box<dyn Transform<'a, TcC = Coordinate<T>>>,
    project_transform: Compose<T>,
    // project_rotate_transform: Box<dyn Transform<'a, TcC = Coordinate<T>>>,
    project_rotate_transform: Compose<T>,
    phi: T, // center
    preclip: Clip<T>,
    postclip: Clip<T>,
    // preclip: Box<
    //     dyn StreamPreClipTrait<
    //         SctC = Coordinate<T>,
    //         SctT = T,
    //         SctOC = Option<Coordinate<T>>,
    //         SctCi = CompareIntersection<T>,
    //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
    //         SpctResample = ResampleEnum<T>,
    //     >,
    // >,
    // postclip: Box<
    //     dyn StreamPostClipTrait<
    //         SpostctStream = StreamSrc,
    //         C = Coordinate<T>,
    //         SctC = Coordinate<T>,
    //         SctT = T,
    //         SctOC = Option<Coordinate<T>>,
    //         SctCi = CompareIntersection<T>,
    //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
    //     >,
    // >,
    x: T,
    y: T, // translate
    lambda: T,
    // rotate: Box<dyn Transform<'a, TcC = Coordinate<T>>>, //rotate, pre-rotate
    rotate: RotateRadiansEnum<T>, //rotate, pre-rotate
    sx: T,                        // reflectX
    sy: T,                        // reflectY
    theta: Option<T>,
    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent
}

// impl<T> Clone for ProjectionMutator<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     fn clone(&self) -> Self {
//         panic!("Must clone");
//         // Self {
//         //     project: self.project.box_clone(),
//         //     project_rotate_transform: self.project_transform.box_clone(),
//         //     rotate: self.rotate.box_clone(),
//         //     ..*self
//         // }
//     }
// }

impl<T> Default for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst + Default,
{
    fn default() -> Self {
        ProjectionMutator::from_projection_raw(
            ProjectionRawEnum::O(OrthographicRaw::default()),
            None,
        )
    }
}
impl<T: CoordFloat + FloatConst + Default> ProjectionMutator<T> {
    pub fn from_projection_raw(
        project: ProjectionRawEnum<T>,
        delta2_p: Option<T>,
    ) -> ProjectionMutator<T> {
        let delta2 = match delta2_p {
            None => {
                T::from(0.5).unwrap() // precision
            }
            Some(delta2) => delta2,
        };

        let pm = ProjectionMutator {
            project,
            alpha: T::zero(), // post-rotate angle
            // cache: None,
            // cache_stream: None,
            // clip_antimeridian: None,
            delta2, // precision
            delta_lambda: T::zero(),
            delta_phi: T::zero(),
            delta_gamma: T::zero(),
            // scale
            k: T::from(150f64).unwrap(),
            // translate
            lambda: T::zero(),
            phi: T::zero(),
            // rotate: Box::new(TransformIdentity::default()), // pre-rot/ate
            rotate: RotateRadiansEnum::I(RotationIdentity::default()), // pre-rotate
            // preclip: Box::new(StreamPreClipNodeStub::default()),
            // postclip: Box::new(StreamPostClipNodeStub::default()),
            preclip: ClipCircle::gen_clip(T::one()), // stub value
            postclip: ClipAntimeridian::gen_clip(),

            sx: T::one(), // reflectX
            sy: T::one(), // reflectX
            theta: None,  // pre-clip angle
            x: T::from(480f64).unwrap(),
            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent
            y: T::from(250).unwrap(),
            // project_resample: gen_resample_node(
            //     TransformIdentity::default(),
            //     Some(delta2),
            // ),
            project_resample: ResampleEnum::R(Resample::default()),
            // project_transform: Box::new(TransformIdentity::default()),
            // project_rotate_transform: Box::new(TransformIdentity::default()),
            project_transform: Compose::default(),
            project_rotate_transform: Compose::default(),
        };

        pm.recenter()
    }

    #[inline]
    fn reset(self) -> ProjectionMutator<T> {
        // self.cache_stream = None;
        // self.cache = None;
        self
    }

    fn recenter(mut self) -> ProjectionMutator<T> {
        let center = ScaleTranslateRotate::new(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            self.alpha,
        )
        .transform(&self.project.transform(&Coordinate {
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

        self.rotate = rotate_radians_transform(self.delta_lambda, self.delta_phi, self.delta_gamma);

        self.project_transform = Compose::new(
            ComposeElemEnum::PRE(self.project.clone()),
            ComposeElemEnum::STR(transform),
        );

        self.project_rotate_transform = Compose::new(
            ComposeElemEnum::RR(self.rotate.clone()),
            ComposeElemEnum::C(Box::new(self.project_transform.clone())),
        );

        // Resample is missing from here.
        self.reset()
    }

    // In javascript stream is used as a property to be removed from the object.
    // In rust that is a closure.
    pub fn stream(
        &self,
        streamDst: StreamDst<T>, // stream: Option<StreamSimpleNode<T>>,
    ) -> StreamTransformRadians<T> {
        // return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
        // return match &self.cache {
        //     Some(c) => Box::new(*c),
        //     None => {
        // self.cache_stream = Some(stream.clone());

        let mut postclip = self.postclip.clone();
        postclip.stream_in(ClipSinkEnum::Src(streamDst));

        let mut resample = self.project_resample.clone();
        resample.stream_postclip_in(postclip);

        let mut preclip = self.preclip.clone();
        preclip.stream_in(ClipSinkEnum::Resample(resample));

        let mut t_rotate_node = StreamTransform::new(Some(self.rotate.clone()));
        t_rotate_node.stream_preclip_in(preclip);

        let mut t_radians_node: StreamTransformRadians<T> = StreamTransformRadians::default();
        t_radians_node.stream_transform_in(t_rotate_node);

        t_radians_node

        //     }
        // };
    }
}

// impl<'a, T> StreamClone for ProjectionMutator<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     type RetType = Box<dyn Stream<C = Coordinate<T>>>;
//     #[inline]
//     fn box_clone(&self) -> Self::RetType {
//         Box::new(*self.clone())
//     }
// }

// impl<'a, T> Stream<T> for ProjectionMutator<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     type C = Coordinate<T>;
// }

// impl<'a, T: CoordFloat + FloatConst + Default> TransformClone<'a> for ProjectionMutator<'a, T> {
//     fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>> {
//         Box::new(self.clone())
//     }
// }

impl<T> Transform for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst + Default,
{
    type TcC = Coordinate<T>;
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

impl<T> Projection<T> for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst + Default,
{
    // #[inline]
    // fn get_preclip(&self) -> StreamPreClipNode<T> {
    //     self.preclip
    // }

    // fn preclip(&mut self, preclip: StreamPRClipNode<T>) {
    //     self.preclip = preclip;
    //     self.theta = None;
    //     return self.reset();
    // }

    // fn get_postclip(&self) -> Option<Box<dyn GeoStream>> {
    //   return self.postclip;
    // }

    // fn postclip(&mut self, postclip: StreamProcessor<T>) {
    //     // self.postclip = postclip;
    //     // self.theta = None;
    //     // return self.reset();
    // }

    // fn get_center(&self) -> Point {
    //   return [self.lambda.to_degrees(), self.phi.to_degrees()];
    // }

    /// TODO dynamic cast and unwrap - Must find a better way.
    // fn center(&mut self, p: Point) {
    //   // self.lambda = (p[0] % F::from_u16(360u16).unwrap()).to_radians();
    //   // self.phi = (p[1] % F::from_u16(360u16).unwrap()).to_radians();
    //   self.recenter();
    // }

    // projection.clipAngle = function(_) {
    //   return arguments.length ? (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) : theta * degrees;
    // };

    #[inline]
    fn get_precision(self) -> T {
        self.delta2.sqrt()
    }

    fn precision(mut self, delta: T) -> ProjectionMutator<T> {
        self.delta2 = delta * delta;
        self.project_resample =
            gen_resample_node(self.project_transform.clone(), Some(self.delta2));
        self.reset()
    }

    // fn get_clip_angle(&self) -> T {}

    fn clip_angle(mut self, angle: StreamOrValueMaybe<T>) -> ProjectionMutator<T> {
        match angle {
            StreamOrValueMaybe::Value(angle) => {
                let theta = angle.to_radians();
                self.theta = Some(theta);
                println!("generating clip circle");
                self.preclip = ClipCircle::gen_clip(theta);
                self
            }
            StreamOrValueMaybe::SP(_preclip) => {
                println!("generatin SP");
                self.theta = None;
                // self.preclip = preclip;
                // self.reset();
                self
            }
            // StreamOrValueMaybe::None => match self.theta {
            //     Some(theta) => Some(theta.to_degrees()),
            //     None => None,
            // },
        }
    }

    fn get_extent(&self) -> Option<[Coordinate<T>; 2]> {
        match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
            }
            _ => None,
        }
    }

    fn extent(self) -> ProjectionMutator<T> {
        // todo!("Must implement.");
        self
    }

    #[inline]
    fn get_scale(&self) -> T {
        self.k
    }

    fn scale(mut self, scale: T) -> ProjectionMutator<T> {
        self.k = scale;
        self.recenter()
    }

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> ProjectionMutator<T> {
        self.x = t.x;
        self.y = t.y;
        self.recenter()
    }

    #[inline]
    fn get_rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_lambda.to_degrees(),
        ]
    }

    fn rotate(mut self, angles: [T; 3]) -> ProjectionMutator<T> {
        let [delta_lambda, delta_phi, delta_gamma] = angles;
        let f360 = T::from(360f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter()
    }
}
