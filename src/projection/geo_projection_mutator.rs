// A collection of functions that mutate a GeoProjection struct.
use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;


use crate::compose::Compose;
use crate::rotation::rotate_radians::rotate_radians;
use crate::resample::Resample;
use crate::Transform;
use crate::clip::antimeridian::ClipAntimeridianState;
// use crate::clip::circle::Circle;

use super::geo_projection::GeoProjection;
use super::geo_stream::GeoStream;
use super::scale_translate::ScaleTranslate;
use super::scale_translate_rotate::ScaleTranslateRotate;

pub struct GeoProjectionMutator<T>
where T: Float {
  pub projection: Box<dyn Transform<T>>,
  k: T, // scale
  x: T,
  y: T, // translate
  lambda: T,
  phi: T, // center
  delta_lambda: T,
  delta_phi: T,
  delta_gamma: T,
  rotate: Option<Box<dyn Transform<T>>>, //rotate, // pre-rotate
  alpha: Option<T>,                 // post-rotate angle
  sx: T,                            // reflectX
  sy: T,                            // reflectY
  theta: Option<T>,
  preclip: Option<Box<dyn Fn(dyn GeoStream)>>,
  postclip: Option<Box<dyn Fn(dyn GeoStream)>>,
  clip_antimeridian: Option<Box<dyn Transform<T>>>,
  x0: Option<T>,
  y0: Option<T>,
  x1: Option<T>,
  y1: Option<T>, // post-clip extent
  delta2: T,     // precision
  project_resample: Option<Box<dyn Transform<T>>>,
  project_transform: Option<Box<dyn Transform<T>>>,
  project_rotate_transform: Option<Box<dyn Transform<T>>>,
  cache_stream: Option<Box<dyn GeoStream>>,
}

impl<T> GeoProjectionMutator<T>
where T: Float + FromPrimitive {
  //TODO set project so recenter can use it.
  // self.project;
  pub fn from_projection_raw(projection: Box<dyn Transform<T>>) -> GeoProjectionMutator<T>
  where T: Float + FromPrimitive {
    return GeoProjectionMutator {
      projection,
      // scale
      k: T::from_u8(150).unwrap(),
      // translate
      x: T::from_u8(480).unwrap(),
      y: T::from_u8(250).unwrap(),
      // center
      lambda: T::zero(),
      phi: T::zero(),
      delta_lambda: T::zero(),
      delta_phi: T::zero(),
      delta_gamma: T::zero(),
      rotate: None, // pre-rotate
      alpha: None,  // post-rotate angle
      sx: T::one(),     // reflectX
      sy: T::one(),     // reflectX
      theta: None,  // clipAntimeridian, // pre-clip angle
      preclip: None,
      postclip: None,
      clip_antimeridian: None,
      x0: None,
      y0: None,
      x1: None,
      y1: None,       //postclip = identity, // post-clip extent
      delta2: T::from(0.5f64).unwrap(), // precision
      project_resample: None,
      project_transform: None,
      project_rotate_transform: None,
      cache_stream: Option::None,
    };
  }

  fn reset(&mut self) {
    self.cache_stream = None;
  }

  fn recenter(&mut self)
  where T: Float + FloatConst {
    let center;
    let transform: Box<dyn Transform<T>>;
    match self.alpha {
      Some(alpha) => {
        center = ScaleTranslateRotate::new(self.k, T::zero(), T::zero(), self.sx, self.sy, alpha)
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
        center = ScaleTranslate::new(self.k, T::zero(), T::zero(), self.sx, self.sy)
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
    self.rotate = Some(rotate_radians(self.delta_lambda, self.delta_phi,self.delta_gamma));
    // self.project_transform = Some(Box::new(Compose{a:self.projection, b:transform}));
    // self.project_rotate_transform = Some(Box::new(Compose{a:self.rotate, b:self.project_transform}));
    // self.project_resample = Some(

    //   Box::new(
    //     Resample::new(self.project_transform, self.delta2)
    //   )

    // );
    // return self.reset();
  }
}

impl<T> GeoProjection<T> for GeoProjectionMutator<T>
where T: Float + FloatConst + FromPrimitive {
  // fn stream(stream: GeoProjection) {
  //   matach cacheStream{
  //     Some(Cache::Stream) => {
  //       cache = transformRadians(
  //         transformRotate(self.rotate)(self.preclip(self.projectResample(self.postclip(cacheStream= stream))))));
  //     }
  //   }
  // }

  // projection.stream = function(stream) {
  //   return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
  // };

  // fn get_preclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.preclip;
  // }

  fn preclip(&mut self, preclip: Option<Box<dyn GeoStream>>) {
    // self.preclip = preclip;
    self.theta = None;
    return self.reset();
  }

  // fn get_postclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.postclip;
  // }

  fn postclip(&mut self, postclip: Option<Box<dyn GeoStream>>) {
    // self.postclip = postclip;
    self.theta = None;
    return self.reset();
  }

  fn get_center(&self) -> [T; 2] {
    return [self.lambda.to_degrees(), self.phi.to_degrees()];
  }

  /// TODO dynamic cast and unwrap - Must find a better way.
  fn center(&mut self, p: [T; 2]) {
    self.lambda = (p[0] % T::from_u8(360).unwrap()).to_radians();
    self.phi = (p[1] % T::from_u8(360).unwrap()).to_radians();
    self.recenter();
  }

  fn get_clip_angle(&self) -> Option<T> {
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


  fn clip_angle(&mut self, angle: Option<T>) {
    match (self.preclip.as_ref(), angle) {
      (Some(preclip),Some(angle))  => {
        self.theta = Some(angle.to_radians());
        // return ClipCircle::new(self.theta);
      },
      (_,_) => {
        self.theta = None;
        // self.preclip = Some(ClipAntimeridianState::new());
        self.reset();
      }

      }
  }

  fn get_scale(&self) -> T {
    return self.k;
  }

  fn scale(&mut self, scale: T) {
    // self.k += scale;
    self.k = self.k + scale;
  }

  fn get_translation(&self) -> [T; 2] {
    return [self.x, self.y];
  }

  fn translate(&mut self, t: [T; 2]) {
    self.x = self.x +  t[0];
    self.y = self.y +  t[1];
    self.recenter();
  }
}

fn generate<T>(raw: Box<dyn Transform<T>>) -> GeoProjectionMutator<T>
where T: Float {
  let mut g = GeoProjectionMutator::from_projection_raw(raw);
  g.recenter();
  return g;
}
