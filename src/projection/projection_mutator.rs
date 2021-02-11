// A collection of functions that mutate a Projection struct.

// A collection of functions that mutate a Projection struct.
// use crate::clip::ClipNodeStub;
use crate::stream::StreamPostClipNodeStub;
use crate::stream::StreamPreClipNodeStub;

use crate::compose::Compose;
use crate::projection::stream_transform::StreamPreclipIn;
use crate::projection::stream_transform::StreamTransform;
use crate::projection::transform_radians::StreamTransformIn;
use crate::projection::transform_radians::StreamTransformRadians;
use crate::projection::transform_radians::StreamTransformRadiansNode;
use crate::projection::transform_radians::StreamTransformRadiansNodeStub;
use crate::rotation::rotate_radians::RotateRadians;
use crate::stream::StreamPostClipNode;
use crate::stream::StreamPostClipTrait;
use crate::stream::StreamPreClipNode;
use crate::stream::StreamPreClipTrait;
use crate::stream::StreamResampleNode;
use crate::stream::StreamResampleTrait;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::rc::Rc;

use crate::stream::Stream;
use crate::stream::StreamSimpleNode;
use crate::Transform;
use crate::TransformIdentity;
// clip generators.
// use crate::clip::antimeridian::generate_antimeridian;
// use crate::clip::circle::generate_circle;

use super::projection::Projection;
use super::projection::StreamOrValueMaybe;

use super::resample::resample::Resample;
use super::scale_translate_rotate::ScaleTranslateRotate;
// use super::path::PathTrait;

pub struct ProjectionMutator<T: CoordFloat + FloatConst> {
    // The mutator lives as long a the proejction it contnains.
    project: Rc<Box<dyn Transform<T>>>,
    alpha: T, // post-rotate angle
    cache: Option<StreamSimpleNode<T>>,
    cache_stream: Option<StreamSimpleNode<T>>,
    // clip_antimeridian: Option<Box<dyn Transform<T>>>,
    delta_lambda: T,
    delta_phi: T,
    delta_gamma: T,
    delta2: T, // precision
    k: T,      // scale
    project_resample: StreamResampleNode<T>,
    project_transform: Rc<Box<dyn Transform<T>>>,
    project_rotate_transform: Rc<Box<dyn Transform<T>>>,
    phi: T, // center
    preclip: StreamPreClipNode<T>,
    postclip: StreamPostClipNode<T>,
    x: T,
    y: T, // translate
    lambda: T,
    rotate: Rc<Box<dyn Transform<T>>>, //rotate, pre-rotate
    sx: T,                             // reflectX
    sy: T,                             // reflectY
    theta: Option<T>,
    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent
}

impl<T: CoordFloat + FloatConst + 'static> ProjectionMutator<T> {
    pub fn from_projection_raw(
        project: Rc<Box<dyn Transform<T>>>,
        delta2_p: Option<T>,
    ) -> ProjectionMutator<T> {
        let delta2 = match delta2_p {
            None => {
                T::from(0.5).unwrap() // precision
            }
            Some(delta2) => delta2,
        };

        let mut pm = ProjectionMutator {
            project: project.clone(),
            alpha: T::zero(), // post-rotate angle
            cache: None,
            cache_stream: None,
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
            rotate: Rc::new(Box::new(TransformIdentity {})), // pre-rotate
            preclip: StreamPreClipNodeStub::new(),
            postclip: StreamPostClipNodeStub::new(),
            sx: T::one(), // reflectX
            sy: T::one(), // reflectX
            theta: None,  // pre-clip angle
            x: T::from(480f64).unwrap(),
            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent
            y: T::from(250).unwrap(),
            project_resample: Resample::gen_node(
                Rc::new(Box::new(TransformIdentity {})),
                Some(delta2),
            ),
            project_transform: Rc::new(Box::new(TransformIdentity {})),
            project_rotate_transform: Rc::new(Box::new(TransformIdentity {})),
        };

        pm.recenter();
        return pm;
    }

    #[inline]
    fn reset(&mut self) {
        self.cache_stream = None;
        self.cache = None;
    }

    fn recenter(&mut self) {
        let center =
            ScaleTranslateRotate::new(self.k, T::zero(), T::zero(), self.sx, self.sy, self.alpha)
                .transform(&self.project.transform(&Coordinate {
                    x: self.lambda,
                    y: self.phi,
                }));

        let transform = Rc::new(ScaleTranslateRotate::new(
            self.k,
            self.x - center.x,
            self.y - center.y,
            self.sx,
            self.sy,
            self.alpha,
        ));

        self.rotate = Rc::new(RotateRadians::new(
            self.delta_lambda,
            self.delta_phi,
            self.delta_gamma,
        ));

        {
            self.project_transform = Rc::new(Compose::new(self.project.clone(), transform));
        }

        self.project_rotate_transform = Rc::new(Compose::new(
            self.rotate.clone(),
            self.project_transform.clone(),
        ));

        // Resample is missing from here.
        self.reset();
    }

    pub fn stream(
        &mut self,
        stream: Option<StreamSimpleNode<T>>,
    ) -> Option<StreamTransformRadiansNode<T>> {
        return match &self.cache {
            None => None,
            Some(c) => {
                // if self.cache_stream == stream {
                if 1 == 0 {
                    // self.cache.clone()
                    Some(StreamTransformRadiansNodeStub::new())
                } else {
                    match stream {
                        None => None,
                        Some(stream) => {
                            self.cache_stream = Some(stream.clone());

                            self.postclip.stream_in(stream);

                            self.project_resample
                                .stream_postclip_in(self.postclip.clone());

                            self.preclip
                                .stream_resample_in(self.project_resample.clone());

                            let mut t_rotate_node =
                                StreamTransform::gen_node(Some(self.rotate.clone()));
                            t_rotate_node.stream_preclip_in(self.preclip.clone());

                            let mut t_radians_node: StreamTransformRadiansNode<T> =
                                StreamTransformRadians::gen_node();
                            t_radians_node.stream_transform_in(t_rotate_node);

                            Some(t_radians_node)
                        }
                    }
                }
            }
        };
    }
}

impl<T> Stream<T> for ProjectionMutator<T> where T: CoordFloat + FloatConst {}

impl<T> Transform<T> for ProjectionMutator<T>
where
    T: CoordFloat + FloatConst,
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
    T: CoordFloat + FloatConst + 'static,
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
            Resample::gen_node(self.project_transform.clone(), Some(self.delta2));
        self.reset();
    }

    fn clip_angle(&mut self, angle: StreamOrValueMaybe<T>) -> Option<T> {
        match angle {
            StreamOrValueMaybe::Value(angle) => {
                let theta = angle.to_radians();
                self.theta = Some(theta);
                println!("generating clip circle");
                // self.preclip = generate_circle(theta);
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
