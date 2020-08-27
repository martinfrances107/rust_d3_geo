// A collection of functions that mutate a Projection struct.

use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::rotation::rotate_radians::RotateRadians;

use crate::transform_stream::StreamProcessor;
use crate::transform_stream::StreamProcessorIdentity;
use crate::transform_stream::TransformStream;
use crate::transform_stream::TransformStreamIdentity;
use crate::Transform;
use crate::TransformIdentity;

// clip generators.
use crate::clip::antimeridian::generate_antimeridian;
use crate::clip::circle::generate_circle;

use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::scale_translate_rotate::ScaleTranslateRotate;

pub struct ProjectionMutator<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  // The mutator lives as long a the proejction it contnains.
  project: Rc<Box<dyn Transform<F>>>,
  alpha: F, // post-rotate angle
  cache: Rc<RefCell<Box<dyn TransformStream<F>>>>,
  cache_stream: Option<Box<dyn TransformStream<F>>>,
  clip_antimeridian: Option<Box<dyn Transform<F>>>,
  delta_lambda: F,
  delta_phi: F,
  delta_gamma: F,
  delta2: F, // precision
  k: F,      // scale
  // project_resample: Rc<StreamProcessor<F>>,
  project_transform: Rc<Box<dyn Transform<F>>>,
  project_rotate_transform: Rc<Box<dyn Transform<F>>>,
  phi: F, // center
  preclip: StreamProcessor<F>,
  postclip: StreamProcessor<F>,
  x: F,
  y: F, // translate
  lambda: F,
  rotate: Rc<Box<dyn Transform<F>>>, //rotate, pre-rotate
  sx: F,                             // reflectX
  sy: F,                             // reflectY
  theta: Option<F>,
  x0: Option<F>,
  y0: Option<F>,
  x1: Option<F>,
  y1: Option<F>, // post-clip extent
}

impl<F> ProjectionMutator<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  pub fn from_projection_raw(project: Rc<Box<dyn Transform<F>>>) -> ProjectionMutator<F>
  where
    F: Float + FloatConst + FromPrimitive,
  {
    let delta2 = F::from(0.5f64).unwrap(); // precision

    let mut pm = ProjectionMutator::<F> {
      project: Rc::clone(&project),
      alpha: F::zero(), // post-rotate angle
      cache: Rc::new(RefCell::new(Box::new(TransformStreamIdentity {}))),
      cache_stream: None,
      clip_antimeridian: None,
      delta2, // precision
      delta_lambda: F::zero(),
      delta_phi: F::zero(),
      delta_gamma: F::zero(),
      // scale
      k: F::from_u8(150u8).unwrap(),
      // translate
      lambda: F::zero(),
      phi: F::zero(),
      rotate: Rc::new(Box::new(TransformIdentity {})), // pre-rotate
      preclip: generate_antimeridian(),
      postclip: StreamProcessorIdentity::new(),
      sx: F::one(), // reflectX
      sy: F::one(), // reflectX
      theta: None,  // pre-clip angle
      x: F::from_u16(480u16).unwrap(),
      x0: None,
      y0: None,
      x1: None,
      y1: None, //postclip = identity, // post-clip extent
      y: F::from_u16(250u16).unwrap(),
      // project_resample,
      project_transform: Rc::new(Box::new(TransformIdentity {})),
      project_rotate_transform: Rc::new(Box::new(TransformIdentity {})),
    };

    pm.recenter();
    return pm;
  }

  fn reset(&mut self) {
    self.cache_stream = None;
  }

  fn recenter(&mut self)
  where
    F: Float + FloatConst + FromPrimitive,
  {
    let center =
      ScaleTranslateRotate::new(self.k, F::zero(), F::zero(), self.sx, self.sy, self.alpha)
        .transform(&self.project.transform(&[self.lambda, self.phi]));

    let transform = ScaleTranslateRotate::new(
      self.k,
      self.x - center[0],
      self.y - center[1],
      self.sx,
      self.sy,
      self.alpha,
    );

    self.rotate = Rc::new(RotateRadians::new(
      self.delta_lambda,
      self.delta_phi,
      self.delta_gamma,
    ));

    {
      self.project_transform = Rc::new(Compose::new(self.project.clone(), Rc::new(transform)));
    }

    self.project_rotate_transform = Rc::new(Compose::new(
      self.rotate.clone(),
      self.project_transform.clone(),
    ));

    // Resample is missing from here.
    self.reset();
  }

  fn stream(
    &mut self,
    stream: Rc<RefCell<Box<dyn TransformStream<F>>>>,
  ) -> Rc<RefCell<Box<dyn TransformStream<F>>>> {
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

impl<F> TransformStream<F> for ProjectionMutator<F> where F: Float + FloatConst + FromPrimitive {}

impl<F> Transform<F> for ProjectionMutator<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn transform(&self, p: &[F; 2]) -> [F; 2] {
    let pt = self.project_rotate_transform.clone();
    let r = [p[0].to_radians(), p[1].to_radians()];
    let out = pt.transform(&r);
    return out;
  }
  fn invert(&self, p: &[F; 2]) -> [F; 2] {
    let pt = self.project_rotate_transform.clone();
    let d = pt.invert(p);
    let out = [d[0].to_degrees(), d[1].to_degrees()];
    return out;
  }
}

impl<F> Projection<F> for ProjectionMutator<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  // fn get_preclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.preclip;
  // }

  fn preclip(&mut self, preclip: StreamProcessor<F>) {
    // self.preclip = preclip;
    // self.theta = None;
    // return self.reset();
  }

  // fn get_postclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.postclip;
  // }

  fn postclip(&mut self, postclip: StreamProcessor<F>) {
    // self.postclip = postclip;
    // self.theta = None;
    // return self.reset();
  }

  // fn get_center(&self) -> [F; 2] {
  //   return [self.lambda.to_degrees(), self.phi.to_degrees()];
  // }

  /// TODO dynamic cast and unwrap - Must find a better way.
  // fn center(&mut self, p: [F; 2]) {
  //   // self.lambda = (p[0] % F::from_u16(360u16).unwrap()).to_radians();
  //   // self.phi = (p[1] % F::from_u16(360u16).unwrap()).to_radians();
  //   self.recenter();
  // }

  // projection.clipAngle = function(_) {
  //   return arguments.length ? (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) : theta * degrees;
  // };

  fn clip_angle(&mut self, angle: StreamProcessorValueMaybe<F>) -> Option<F>
  where
    F: Float + FloatConst,
  {
    match angle {
      StreamProcessorValueMaybe::Value(angle) => {
        let theta = angle.to_radians();
        self.theta = Some(theta);
        println!("generating clip circle");
        self.preclip = generate_circle(theta);
        None
      }
      StreamProcessorValueMaybe::SP(preclip) => {
        println!("generatin SP");
        self.theta = None;
        self.preclip = preclip;
        // self.reset();
        None
      }
      StreamProcessorValueMaybe::None => match self.theta {
        Some(theta) => Some(theta.to_degrees()),
        None => None,
      },
    }
  }

  fn scale(&mut self, scale: Option<&F>) {
    match scale {
      Some(scale) => {
        self.k = *scale;
        self.recenter();
      }
      None => {}
    }
  }

  fn translate(&mut self, t: Option<&[F; 2]>) -> Option<[F; 2]> {
    match t {
      Some(t) => {
        self.x = t[0];
        self.y = t[1];
        self.recenter();
        return None;
      }
      None => {
        return Some([self.x, self.y]);
      }
    }
  }

  fn rotate(&mut self, angles: Option<[F; 3]>) -> Option<[F; 3]> {
    return match angles {
      Some(angles) => {
        let [delta_lambda, delta_phi, delta_gamma] = angles;
        let f360 = F::from(360u16).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter();
        None
      }
      None => {
        return Some([
          self.delta_lambda.to_degrees(),
          self.delta_phi.to_degrees(),
          self.delta_lambda.to_degrees(),
        ]);
      }
    };
  }
}
