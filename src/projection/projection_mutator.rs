// A collection of functions that mutate a Projection struct.

use std::cell::{RefCell};
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::resample::gen_resample;
use crate::rotation::rotate_radians::RotateRadians;
// use crate::stream::Stream;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::StreamProcessorIdentity;
use crate::transform_stream::TransformStreamIdentity;
use crate::transform_stream::TransformStream;
use crate::Transform;
use crate::TransformIdentity;
// use crate::clip::antimeridian::ClipAntimeridianState;
use crate::clip::antimeridian::generate_antimeridian;
// use crate::clip::circle::Circle;

use super::projection::Projection;
// use super::stream_wrapper::StreamWrapper;
// use super::scale_translate::ScaleTranslate;
use super::scale_translate_rotate::ScaleTranslateRotate;
use super::transform_radians::TransformRadians;
use super::transform_rotate::TransformRotate;
// use super::stream_wrapper::StreamWrapper;

pub struct ProjectionMutator<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  // The mutator lives as long a the proejction it contnains.
  // pub projection: RefCell<Box<dyn Transform<F>>>,
  alpha: F, // post-rotate angle
  cache: Rc<RefCell<Box<dyn TransformStream<F>>>>,
  cache_stream: Option<Box<dyn TransformStream<F>>>,
  clip_antimeridian: Option<Box<dyn Transform<F>>>,
  delta_lambda: F,
  delta_phi: F,
  delta_gamma: F,
  delta2: Option<F>, // precision
  k: F,      // scale
  project_resample: Rc<RefCell<StreamProcessor<F>>>,
  // project_transform: Box<dyn Transform<F>>,
  // project_rotate_transform: Box<dyn Transform<F>>,
  phi: F, // center
  preclip: StreamProcessor<F>,
  postclip: StreamProcessor<F>,
  x: F,
  y: F, // translate
  lambda: F,
  rotate: Rc<RefCell<Box<dyn Transform<F>>>>, //rotate, // pre-rotate
  sx: F,                         // reflectX
  sy: F,                         // reflectY
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
  pub fn from_projection_raw(
    projection: Box<dyn Transform<F>>,
    alpha: Option<F>,
    k: Option<F>,
    x: Option<F>,
    y: Option<F>,
    sx: Option<F>,
    sy: Option<F>,
    lambda: Option<F>,
    phi: Option<F>,
    delta_lambda: Option<F>,
    delta_phi: Option<F>,
    delta_gamma: Option<F>,
    delta2: Option<F>,
  ) -> ProjectionMutator<F>
  where
    F: Float + FloatConst + FromPrimitive,
  {
    let alpha = alpha.unwrap_or(F::zero());
    let delta_lambda = delta_lambda.unwrap_or(F::zero());
    let delta_phi = delta_phi.unwrap_or(F::zero());
    let delta_gamma = delta_gamma.unwrap_or(F::zero());
    let delta2 = Some(F::from(0.5f64).unwrap()); // precision
    // let delta2 = delta2.unwrap_or(F::from(0.5f64).unwrap()); // precision
                                                             // projection: RefCell::new(projection),
                                                             // scale
    let k = k.unwrap_or(F::from_u8(150u8).unwrap());
    // translate
    // center
    let lambda = lambda.unwrap_or(F::zero());
    let phi = phi.unwrap_or(F::zero());
    let center;
    let transform: Box<dyn Transform<F>>;
    let x = x.unwrap_or(F::from_u16(480u16).unwrap());
    let y = y.unwrap_or(F::from_u16(250u16).unwrap());
    let sx = sx.unwrap_or(F::one());
    let sy = sy.unwrap_or(F::one());

    center = ScaleTranslateRotate::new(k, F::zero(), F::zero(), sx, sy, alpha)
      .transform(&projection.transform(&[lambda, phi]));
    transform = ScaleTranslateRotate::new(k, x - center[0], y - center[1], sx, sy, alpha);

    let rotate = RotateRadians::new().rotate_radians(delta_lambda, delta_phi, delta_gamma);
    let project_transform = Rc::new(RefCell::new(Compose::new(projection, transform)));
    // let project_rotate_transform = Box::new(Compose::new(rotate, project_transform));
    // let project_rotate_transform = Box::new(Compose{a: rotate, b:project_transform});
    // self.project_resample = Some(

      // projectResample = resample(projectTransform, delta2 = _ * _)

    let project_resample = gen_resample(project_transform, delta2);

    let mut pm = ProjectionMutator::<F> {
      alpha, // post-rotate angle
      cache: Rc::new(RefCell::new(Box::new(TransformStreamIdentity{}))),
      cache_stream: None,
      clip_antimeridian: None,
      delta2, // precision
      delta_lambda,
      delta_phi,
      delta_gamma,
      // projection: RefCell::new(projection),
      // scale
      k,
      // translate
      // center
      lambda,
      phi,
      rotate: Rc::new(RefCell::new(Box::new(TransformIdentity {}))), // pre-rotate
      preclip: generate_antimeridian(),
      postclip: StreamProcessorIdentity::new(),
      sx,          // reflectX
      sy,          // reflectX
      theta: None, // pre-clip angle
      x,
      x0: None,
      y0: None,
      x1: None,
      y1: None, //postclip = identity, // post-clip extent
      y,
      project_resample,
      // project_transform: Box::new(TransformIdentity {}),
      // project_rotate_transform,
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
    // let center;
    // let transform: Box<dyn Transform<F>>;
    // let p = self.projection.borrow();
    //     center = ScaleTranslateRotate::new(self.k, F::zero(), F::zero(), self.sx, self.sy, self.alpha.unwrap_or(F::zero()))
    //       .transform(&p.transform(&[self.lambda, self.phi]));
    //     transform = ScaleTranslateRotate::new(
    //       self.k,
    //       self.x - center[0],
    //       self.y - center[1],
    //       self.sx,
    //       self.sy,
    //       self.alpha.unwrap_or(F::zero()),
    //     );

    // self.rotate = rotate_radians(self.delta_lambda, self.delta_phi,self.delta_gamma);
    // // let p2 = self.projection.take();
    // self.project_transform = Box::new(Compose::new(p2, transform));
    // self.project_rotate_transform = Box::new(Compose{a: self.rotate, b:self.project_transform});
    // self.project_rotate_transform = Box::new(Compose{a: self.rotate, b:self.project_transform});
    // self.project_resample = Some(

    //   Box::new(
    //     Resample::new(self.project_transform, self.delta2)
    //   )

    // );
    // return self.reset();
  }

  fn stream(&mut self, stream: Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Rc<RefCell<Box<dyn TransformStream<F>>>> {

    let resample  = self.project_resample.borrow_mut();

    // post clip is just the identity stream in stereographic tests.
    // let post_clip_s = self.postclip.stream(stream);
    let resample_out = resample(stream);
    let preclip_out = (self.preclip)(resample_out);
    let t_rotate = TransformRotate::new(self.rotate.clone());
    let t_rotate_out = t_rotate(preclip_out);
    let t_radians = TransformRadians::new();
    let t_radians_out = t_radians(t_rotate_out);

    self.cache = t_radians_out;

    return self.cache.clone();
  }

}

impl<F> TransformStream<F> for ProjectionMutator<F>
where
  F: Float + FloatConst + FromPrimitive,
{

}

impl<F> Transform<F> for ProjectionMutator<F>
where F: Float + FloatConst + FromPrimitive {

}

impl<F> Projection<F> for ProjectionMutator<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  // projection.stream = function(stream) {
  //   return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
  // };

  // fn get_preclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.preclip;
  // }

  fn preclip(&mut self, preclip: StreamProcessor<F>) {
    // self.preclip = preclip;
    // self.theta = None;
    return self.reset();
  }

  // fn get_postclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.postclip;
  // }

  fn postclip(&mut self, postclip: StreamProcessor<F>) {
    // self.postclip = postclip;
    // self.theta = None;
    return self.reset();
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

  // fn get_clip_angle(&self) -> Option<F> {
  // return match self.theta {
  //   Some(theta) => Some(theta.to_degrees()),
  //   None => None,
  // };
  // }

  // projection.clipAngle = function(_) {
  //   return arguments.length ? (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) : theta * degrees;
  // };

  // projection.clipAngle = function(_) {
  //   return (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) ;
  // };

  fn clip_angle(&mut self, angle: Option<F>) {
    match angle {
      Some(angle) => {
        // self.theta = Some(angle.to_radians());
        // return ClipCircle<F>::new(self.theta);
      }
      None => {
        // self.theta = None;
        // self.preclip = generate_antimeridian<F>();
        self.reset();
      }
    }
  }

  // fn get_scale(&self) -> F {
  //   return self.k;
  // }

  // fn scale(&mut self, scale: &F) {
  //   self.k = self.k + *scale;
  // }

  fn get_translation(&self) -> [F; 2] {
    return [self.x, self.y];
  }

  // fn translate(&mut self, t: &[F; 2]) {
  //   self.x = self.x +  t[0];
  //   self.y = self.y +  t[1];
  //   self.recenter();
  // }
}

// fn generate<'a, F: 'static>(raw: Box<dyn Transform<F>>) -> ProjectionMutator<'a, F>
// where F: Float + FloatConst + FromPrimitive {
//   let mut g = ProjectionMutator::<'a, F>::from_projection_raw(&raw);
//   g.recenter();
//   return g;
// }
