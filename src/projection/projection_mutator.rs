use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// A collection of functions that mutate a Projection struct.

use super::projection::Projection;
use super::projection::StreamOrValueMaybe;
use crate::compose::Compose;
use crate::projection::resample::gen_resample_node;
use crate::projection::resample::ResampleEnum;
// use crate::projection::resample::Resample;
// use crate::projection::resample::ResampleNone;

use crate::projection::stream_transform::StreamPreclipIn;
use crate::projection::stream_transform::StreamTransform;
use crate::projection::stream_transform_radians::StreamTransformIn;
use crate::projection::stream_transform_radians::StreamTransformRadians;
// use crate::projection::stream_transform_radians::StreamTransformRadiansNode;
// use crate::projection::stream_transform_radians::StreamTransformRadiansNodeStub;

use crate::clip::clip::ClipSinkEnum;

use crate::rotation::rotate_radians_transform::rotate_radians_transform;
use crate::stream::CompareIntersection;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::stream::StreamSrc;
// use crate::stream::StreamPostClipNode;
use crate::stream::stream_postclip_node_stub::StreamPostClipNodeStub;
use crate::stream::StreamPostClipTrait;
// use crate::stream::StreamPreClipNode;
use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::circle::ClipCircle;
use crate::stream::stream_preclip_node_stub::StreamPreClipNodeStub;
use crate::stream::StreamPreClipTrait;
// use crate::stream::StreamResampleNode;
// use crate::stream::StreamResampleTrait;
use super::resample::StreamResampleTrait;
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
// use super::resample::resample::Resample;
// use super::resample::resample_none::ResampleNone;
// use crate::stream::StreamSourceDummy;

// #[derive(Clone)]
pub struct ProjectionMutator<T: CoordFloat + FloatConst + Default + 'static> {
    // The mutator lives as long a the proejction it contnains.
    project: Box<dyn Transform<TcC = Coordinate<T>>>,
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
    project_transform: Box<dyn Transform<TcC = Coordinate<T>>>,
    project_rotate_transform: Box<dyn Transform<TcC = Coordinate<T>>>,
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
    rotate: Box<dyn Transform<TcC = Coordinate<T>>>, //rotate, pre-rotate
    sx: T,                                           // reflectX
    sy: T,                                           // reflectY
    theta: Option<T>,
    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent
}

impl<T> Clone for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn clone(&self) -> Self {
        panic!("Must clone");
        // Self {
        //     project: self.project.box_clone(),
        //     project_rotate_transform: self.project_transform.box_clone(),
        //     rotate: self.rotate.box_clone(),
        //     ..*self
        // }
    }
}

impl<'a, T: CoordFloat + FloatConst + Default + 'static> ProjectionMutator<T> {
    pub fn from_projection_raw(
        project: Box<dyn Transform<TcC = Coordinate<T>>>,
        delta2_p: Option<T>,
    ) -> ProjectionMutator<T> {
        let delta2 = match delta2_p {
            None => {
                T::from(0.5).unwrap() // precision
            }
            Some(delta2) => delta2,
        };

        let mut pm = ProjectionMutator {
            project: project.box_clone(),
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
            rotate: Box::new(TransformIdentity::default()), // pre-rotate
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
            project_resample: gen_resample_node(
                Box::new(TransformIdentity::default()),
                Some(delta2),
            ),
            project_transform: Box::new(TransformIdentity::default()),
            project_rotate_transform: Box::new(TransformIdentity::default()),
        };

        pm.recenter();
        return pm;
    }

    #[inline]
    fn reset(&mut self) {
        // self.cache_stream = None;
        // self.cache = None;
    }

    fn recenter(&mut self) {
        let center =
            ScaleTranslateRotate::new(self.k, T::zero(), T::zero(), self.sx, self.sy, self.alpha)
                .transform(&self.project.transform(&Coordinate {
                    x: self.lambda,
                    y: self.phi,
                }));

        let transform = ScaleTranslateRotate::new(
            self.k,
            self.x - center.x,
            self.y - center.y,
            self.sx,
            self.sy,
            self.alpha,
        );

        self.rotate = rotate_radians_transform(self.delta_lambda, self.delta_phi, self.delta_gamma);

        {
            self.project_transform = Compose::new(self.project.box_clone(), transform.box_clone());
        }

        self.project_rotate_transform =
            Compose::new(self.rotate.box_clone(), self.project_transform.box_clone());

        // Resample is missing from here.
        self.reset();
    }

    // In javascript stream is used as a property to be removed from the object.
    // In rust that is a closure.
    pub fn stream(
        &mut self,
        // stream: Option<StreamSimpleNode<T>>,
    ) -> Box<dyn Fn(StreamSrc<T>) -> StreamTransformRadians<T> + '_> {
        // return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
        // return match &self.cache {
        //     Some(c) => Box::new(*c),
        //     None => {
        // self.cache_stream = Some(stream.clone());
        Box::new(move |stream: StreamSrc<T>| {
            let mut postclip = self.postclip.clone();
            postclip.stream_in(ClipSinkEnum::Src(stream));

            let mut resample = self.project_resample.clone();
            resample.stream_postclip_in(postclip);

            let mut preclip = self.preclip.clone();
            preclip.stream_in(ClipSinkEnum::Resample(resample));

            let mut t_rotate_node = StreamTransform::new(Some(self.rotate.box_clone()));
            t_rotate_node.stream_preclip_in(preclip);

            let mut t_radians_node: StreamTransformRadians<T> = StreamTransformRadians::default();
            t_radians_node.stream_transform_in(t_rotate_node);

            t_radians_node
        })
        //     }
        // };
    }
}

impl<T> StreamClone for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}
impl<T> Stream for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
}

impl<T: CoordFloat + FloatConst + Default + 'static> TransformClone for ProjectionMutator<T> {
    type TcC = Coordinate<T>;
    fn box_clone(&self) -> Box<dyn Transform<TcC = Self::TcC>> {
        Box::new(self.clone())
    }
}

impl<T> Transform for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
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
    T: CoordFloat + FloatConst + Default + 'static,
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

    fn precision(&mut self, delta: T) {
        self.delta2 = delta * delta;
        self.project_resample =
            gen_resample_node(self.project_transform.box_clone(), Some(self.delta2));
        self.reset();
    }

    fn clip_angle(&mut self, angle: StreamOrValueMaybe<T>) -> Option<T> {
        match angle {
            StreamOrValueMaybe::Value(angle) => {
                let theta = angle.to_radians();
                self.theta = Some(theta);
                println!("generating clip circle");
                self.preclip = ClipCircle::gen_clip(theta);
                None
            }
            StreamOrValueMaybe::SP(preclip) => {
                println!("generatin SP");
                self.theta = None;
                // self.preclip = preclip;
                // self.reset();
                None
            }
            StreamOrValueMaybe::None => match self.theta {
                Some(theta) => Some(theta.to_degrees()),
                None => None,
            },
        }
    }

    #[inline]
    fn get_scale(&self) -> T {
        self.k
    }

    fn scale(&mut self, scale: T) {
        self.k = scale;
        self.recenter();
    }

    fn translate(&mut self, t: Option<&Coordinate<T>>) -> Option<Coordinate<T>> {
        return match t {
            Some(t) => {
                self.x = t.x;
                self.y = t.y;
                self.recenter();
                None
            }
            None => Some(Coordinate {
                x: self.x,
                y: self.y,
            }),
        };
    }

    fn rotate(&mut self, angles: Option<[T; 3]>) -> Option<[T; 3]> {
        return match angles {
            Some(angles) => {
                let [delta_lambda, delta_phi, delta_gamma] = angles;
                let f360 = T::from(360f64).unwrap();
                self.delta_lambda = (delta_lambda % f360).to_radians();
                self.delta_phi = (delta_phi % f360).to_radians();
                self.delta_gamma = (delta_gamma % f360).to_radians();
                self.recenter();
                None
            }
            None => Some([
                self.delta_lambda.to_degrees(),
                self.delta_phi.to_degrees(),
                self.delta_lambda.to_degrees(),
            ]),
        };
    }
}
