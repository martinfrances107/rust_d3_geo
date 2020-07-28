// A collection of functions that mutate a GeoProjection struct.
use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;


use crate::compose::Compose;
use crate::resample::Resample;
use crate::rotation::rotate_radians::rotate_radians;
use crate::stream::GeoStream;
use crate::Transform;
use crate::TransformIdentity;
// use crate::clip::antimeridian::ClipAntimeridianState;
use crate::clip::antimeridian::generate_antimeridian;
// use crate::clip::circle::Circle;

use super::geo_projection::GeoProjection;
use super::geo_stream_wrapper::GeoStreamWrapper;
use super::scale_translate::ScaleTranslate;
use super::scale_translate_rotate::ScaleTranslateRotate;

pub struct GeoProjectionMutator<'a, F>
where F: Float + FromPrimitive{
  // The mutator lives as long a the proejction it contnains.
  pub projection: &'a Box<dyn Transform<F>>,
  k: F, // scale
  x: F,
  y: F, // translate
  lambda: F,
  phi: F, // center
  delta_lambda: F,
  delta_phi: F,
  delta_gamma: F,
  rotate: Box<dyn Transform<F>>, //rotate, // pre-rotate
  alpha: Option<F>,                 // post-rotate angle
  sx: F,                            // reflectX
  sy: F,                            // reflectY
  theta: Option<F>,
  preclip: Box<dyn Fn(dyn GeoStream<F>)>,
  postclip: Option<Box<dyn Fn(dyn GeoStream<F>)>>,
  clip_antimeridian: Option<Box<dyn Transform<F>>>,
  x0: Option<F>,
  y0: Option<F>,
  x1: Option<F>,
  y1: Option<F>, // post-clip extent
  delta2: F,     // precision
  project_resample: Box<dyn Transform<F>>,
  project_transform: Box<dyn Transform<F>>,
  project_rotate_transform: Box<dyn Transform<F>>,
  cache_stream: Option<Box<dyn GeoStream<F>>>,
}

impl<'a, F: 'static> GeoProjectionMutator<'a, F>
where F: Float + FromPrimitive {
  pub fn from_projection_raw(projection: &'a Box<dyn Transform<F>>) -> GeoProjectionMutator<F>
  where F: Float + FromPrimitive {
    return GeoProjectionMutator {
      projection,
      // scale
      k: F::from_u8(150u8).unwrap(),
      // translate
      x: F::from_u16(480u16).unwrap(),
      y: F::from_u16(250u16).unwrap(),
      // center
      lambda: F::zero(),
      phi: F::zero(),
      delta_lambda: F::zero(),
      delta_phi: F::zero(),
      delta_gamma: F::zero(),
      rotate: Box::new(TransformIdentity{}), // pre-rotate
      alpha: None,  // post-rotate angle
      sx: F::one(),     // reflectX
      sy: F::one(),     // reflectX
      theta: None,  // clipAntimeridian, // pre-clip angle
      preclip:  None,
      postclip: None,
      clip_antimeridian: None,
      x0: None,
      y0: None,
      x1: None,
      y1: None,       //postclip = identity, // post-clip extent
      delta2: F::from(0.5f64).unwrap(), // precision
      project_resample: Box::new(TransformIdentity{}),
      project_transform: Box::new(TransformIdentity{}),
      project_rotate_transform: Box::new(TransformIdentity{}),
      cache_stream: Option::None,
    };
  }

  fn reset(&mut self) {
    self.cache_stream = None;
  }

  fn recenter(&mut self)
  where F: Float + FloatConst {
    let center;
    let transform: Box<dyn Transform<F>>;
    match self.alpha {
      Some(alpha) => {
        center = ScaleTranslateRotate::new(self.k, F::zero(), F::zero(), self.sx, self.sy, alpha)
          .transform(&[self.lambda, self.phi]);
        transform = ScaleTranslateRotate::new(
          self.k,
          self.x - center[0],
          self.y - center[1],
          self.sx,
          self.sy,
          alpha,
        );
      }
      None => {
        center = ScaleTranslate::new(self.k, F::zero(), F::zero(), self.sx, self.sy)
          .transform(&[self.lambda, self.phi]);
        transform = ScaleTranslate::new(
          self.k,
          self.x - center[0],
          self.y - center[1],
          self.sx,
          self.sy,
        );
      }
    };
    self.rotate = rotate_radians(&self.delta_lambda, &self.delta_phi,&self.delta_gamma);
    self.project_transform = Box::new(Compose{a:*self.projection, b:transform});
    self.project_rotate_transform = Box::new(Compose{a: self.rotate, b:self.project_transform});
    self.project_rotate_transform = Box::new(Compose{a: self.rotate, b:self.project_transform});
    // self.project_resample = Some(

    //   Box::new(
    //     Resample::new(self.project_transform, self.delta2)
    //   )

    // );
    // return self.reset();
  }
}

impl<'a, F: 'static > GeoStreamWrapper<F> for GeoProjectionMutator<'a, F>
where F: Float + FloatConst + FromPrimitive {
  fn stream(stream: dyn GeoStream<F>) ->  dyn GeoStream<F> {

        // cache = transformRadians (
        //   // transformRotate(self.rotate)(self.preclip(self.projectResample(self.postclip(cacheStream= stream))))
        // );

    return stream;
  }
}



impl<'a, F: 'static > GeoProjection<F> for GeoProjectionMutator<'a, F>
where F: Float + FloatConst + FromPrimitive {
  // fn stream(stream: GeoProjection) {
  //   matach cacheStream {
  //     Some(Cache::Stream) => {
  //       cache = transformRadians (
  //         transformRotate(self.rotate)(self.preclip(self.projectResample(self.postclip(cacheStream= stream)))));
  //     },
  //     None => {}
  //   }
  // }

  // projection.stream = function(stream) {
  //   return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
  // };

  // fn get_preclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.preclip;
  // }

  fn preclip(&mut self, preclip: Option<Box<dyn GeoStream<F>>>) {
    // self.preclip = preclip;
    self.theta = None;
    return self.reset();
  }

  // fn get_postclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.postclip;
  // }

  fn postclip(&mut self, postclip: Option<Box<dyn GeoStream<F>>>) {
    // self.postclip = postclip;
    self.theta = None;
    return self.reset();
  }

  fn get_center(&self) -> [F; 2] {
    return [self.lambda.to_degrees(), self.phi.to_degrees()];
  }

  /// TODO dynamic cast and unwrap - Must find a better way.
  fn center(&mut self, p: [F; 2]) {
    self.lambda = (p[0] % F::from_u16(360u16).unwrap()).to_radians();
    self.phi = (p[1] % F::from_u16(360u16).unwrap()).to_radians();
    self.recenter();
  }

  fn get_clip_angle(&self) -> Option<F> {
    return match self.theta {
      Some(theta) => Some(theta.to_degrees()),
      None => None,
    };
  }

  // projection.clipAngle = function(_) {
  //   return arguments.length ? (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) : theta * degrees;
  // };

  // projection.clipAngle = function(_) {
  //   return (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) ;
  // };


  fn clip_angle(&mut self, angle: Option<F>) {
    match (self.preclip.as_ref(), angle) {
      (Some(preclip),Some(angle))  => {
        self.theta = Some(angle.to_radians());
        // return ClipCircle<F>::new(self.theta);
      },
      (_,_) => {
        self.theta = None;
        // self.preclip = Some(ClipAntimeridianState::new());
        self.preclip=Some(generate_antimeridian());
        self.reset();
      }

      }
  }

  fn get_scale(&self) -> F {
    return self.k;
  }

  fn scale(&mut self, scale: &F) {
    self.k = self.k + *scale;
  }

  fn get_translation(&self) -> [F; 2] {
    return [self.x, self.y];
  }

  fn translate(&mut self, t: &[F; 2]) {
    self.x = self.x +  t[0];
    self.y = self.y +  t[1];
    self.recenter();
  }
}

fn generate<F: 'static>(raw: &Box<dyn Transform<F>>) -> GeoProjectionMutator<F>
where F: Float + FloatConst + FromPrimitive {
  let mut g = GeoProjectionMutator::<F>::from_projection_raw(raw);
  g.recenter();
  return g;
}
