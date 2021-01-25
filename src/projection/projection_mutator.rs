// A collection of functions that mutate a Projection struct.

use std::cell::RefCell;

// A collection of functions that mutate a Projection struct.
use crate::rotation::rotate_radians::RotateRadians;
use crate::{compose::Compose, transform_stream::StreamProcessor};
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::rc::Rc;

use crate::stream::Stream;
use crate::transform_stream::StreamIdentity;

use crate::Transform;
use crate::TransformIdentity;

// clip generators.
// use crate::clip::antimeridian::generate_antimeridian;
// use crate::clip::circle::generate_circle;

use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::scale_translate_rotate::ScaleTranslateRotate;

pub struct ProjectionMutator<T: CoordFloat + FloatConst> {
    // The mutator lives as long a the proejction it contnains.
    project: Rc<Box<dyn Transform<T>>>,
    alpha: T, // post-rotate angle
    cache: Rc<RefCell<Box<dyn Stream<T>>>>,
    cache_stream: Option<Box<dyn Stream<T>>>,
    // clip_antimeridian: Option<Box<dyn Transform<T>>>,
    delta_lambda: T,
    delta_phi: T,
    delta_gamma: T,
    delta2: T, // precision
    k: T,      // scale
    // project_resample: Rc<StreamProcessor<F>>,
    project_transform: Rc<Box<dyn Transform<T>>>,
    project_rotate_transform: Rc<Box<dyn Transform<T>>>,
    phi: T, // center
    // preclip: StreamProcessor<T>,
    // postclip: StreamProcessor<T>,
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
    pub fn from_projection_raw(project: Rc<Box<dyn Transform<T>>>) -> ProjectionMutator<T> {
        let delta2 = T::from(0.5).unwrap(); // precision

        let mut pm = ProjectionMutator {
            project: Rc::clone(&project),
            alpha: T::zero(), // post-rotate angle
            cache: Rc::new(RefCell::new(Box::new(StreamIdentity {}))),
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
            // preclip: generate_antimeridian(),
            // postclip: StreamProcessorIdentity::new(),
            sx: T::one(), // reflectX
            sy: T::one(), // reflectX
            theta: None,  // pre-clip angle
            x: T::from(480f64).unwrap(),
            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent
            y: T::from(250).unwrap(),
            // project_resample,
            project_transform: Rc::new(Box::new(TransformIdentity {})),
            project_rotate_transform: Rc::new(Box::new(TransformIdentity {})),
        };

        pm.recenter();
        return pm;
    }

    #[inline]
    fn reset(&mut self) {
        self.cache_stream = None;
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
        stream: Rc<RefCell<Box<dyn Stream<T>>>>,
    ) -> Rc<RefCell<Box<dyn Stream<T>>>> {
        // let resample = self.project_resample.borrow_mut();

        // post clip is just the identity stream in stereographic tests.
        // let post_clip_s = self.postclip.stream(stream);
        // let resample_out = resample(stream);
        // let preclip_out = (self.preclip)(resample_out);
        // let t_rotate = TransformRotate::new(self.rotate.clone());
        // let t_rotate_out = t_rotate(preclip_out);
        // let t_radians = TransformRadians::new();
        // let t_radians_out = t_radians(t_rotate_out);

        // self.cache = t_radians_out;
        return self.cache.clone();
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
        let out = self.project_rotate_transform.transform(&r);
        return out;
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let d = self.project_rotate_transform.invert(p);
        let out = Coordinate {
            x: d.x.to_degrees(),
            y: d.y.to_degrees(),
        };
        return out;
    }
}

impl<T: CoordFloat + FloatConst + 'static> Projection<T> for ProjectionMutator<T> {
    // fn get_preclip(&self) -> Option<Box<dyn GeoStream>> {
    //   return self.preclip;
    // }

    // fn preclip(&mut self, preclip: StreamProcessor<T>) {
    //     // self.preclip = preclip;
    //     // self.theta = None;
    //     // return self.reset();
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

    fn clip_angle(&mut self, angle: StreamProcessorValueMaybe<T>) -> Option<T> {
        match angle {
            StreamProcessorValueMaybe::Value(angle) => {
                let theta = angle.to_radians();
                self.theta = Some(theta);
                println!("generating clip circle");
                // self.preclip = generate_circle(theta);
                None
            }
            StreamProcessorValueMaybe::SP(preclip) => {
                println!("generatin SP");
                self.theta = None;
                // self.preclip = preclip;
                // self.reset();
                None
            }
            StreamProcessorValueMaybe::None => match self.theta {
                Some(theta) => Some(theta.to_degrees()),
                None => None,
            },
        }
    }

    fn scale(&mut self, scale: Option<&T>) {
        match scale {
            Some(scale) => {
                self.k = *scale;
                self.recenter();
            }
            None => {}
        }
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
