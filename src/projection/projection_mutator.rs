// A collection of functions that mutate a Projection struct.

use std::cell::RefCell;
use std::rc::Rc;

use delaunator::Point;

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

pub struct ProjectionMutator
{
  // The mutator lives as long a the proejction it contnains.
  project: Rc<Box<dyn Transform>>,
  alpha: f64, // post-rotate angle
  cache: Rc<RefCell<Box<dyn TransformStream>>>,
  cache_stream: Option<Box<dyn TransformStream>>,
  clip_antimeridian: Option<Box<dyn Transform>>,
  delta_lambda: f64,
  delta_phi: f64,
  delta_gamma: f64,
  delta2: f64, // precision
  k: f64,      // scale
  // project_resample: Rc<StreamProcessor<F>>,
  project_transform: Rc<Box<dyn Transform>>,
  project_rotate_transform: Rc<Box<dyn Transform>>,
  phi: f64, // center
  preclip: StreamProcessor,
  postclip: StreamProcessor,
  x: f64,
  y: f64, // translate
  lambda: f64,
  rotate: Rc<Box<dyn Transform>>, //rotate, pre-rotate
  sx: f64,                             // reflectX
  sy: f64,                             // reflectY
  theta: Option<f64>,
  x0: Option<f64>,
  y0: Option<f64>,
  x1: Option<f64>,
  y1: Option<f64>, // post-clip extent
}

impl ProjectionMutator
{
  pub fn from_projection_raw(project: Rc<Box<dyn Transform>>) -> ProjectionMutator
  {
    let delta2 = 0.5f64; // precision

    let mut pm = ProjectionMutator{
      project: Rc::clone(&project),
      alpha: 0f64, // post-rotate angle
      cache: Rc::new(RefCell::new(Box::new(TransformStreamIdentity {}))),
      cache_stream: None,
      clip_antimeridian: None,
      delta2, // precision
      delta_lambda: 0f64,
      delta_phi: 0f64,
      delta_gamma: 0f64,
      // scale
      k: 150f64,
      // translate
      lambda: 0f64,
      phi: 0f64,
      rotate: Rc::new(Box::new(TransformIdentity{})), // pre-rotate
      preclip: generate_antimeridian(),
      postclip: StreamProcessorIdentity::new(),
      sx: 1f64, // reflectX
      sy: 1f64, // reflectX
      theta: None,  // pre-clip angle
      x: 480f64,
      x0: None,
      y0: None,
      x1: None,
      y1: None, //postclip = identity, // post-clip extent
      y: 250f64,
      // project_resample,
      project_transform: Rc::new(Box::new(TransformIdentity{})),
      project_rotate_transform: Rc::new(Box::new(TransformIdentity{})),
    };

    pm.recenter();
    return pm;
  }

  fn reset(&mut self) {
    self.cache_stream = None;
  }

  fn recenter(&mut self)
  {
    let center =
      ScaleTranslateRotate::new(self.k, 0f64, 0f64, self.sx, self.sy, self.alpha)
        .transform(&self.project.transform(&Point{x:self.lambda, y:self.phi}));

    let transform = ScaleTranslateRotate::new(
      self.k,
      self.x - center.x,
      self.y - center.y,
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
    stream: Rc<RefCell<Box<dyn TransformStream>>>,
  ) -> Rc<RefCell<Box<dyn TransformStream>>> {
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

impl TransformStream for ProjectionMutator{}

impl Transform for ProjectionMutator
{
  fn transform(&self, p: &Point) -> Point {
    let pt = self.project_rotate_transform.clone();
    let r = Point{x:p.x.to_radians(), y:p.y.to_radians()};
    let out = pt.transform(&r);
    return out;
  }
  fn invert(&self, p: &Point) -> Point {
    let pt = self.project_rotate_transform.clone();
    let d = pt.invert(p);
    let out = Point{x:d.x.to_degrees(), y:d.y.to_degrees()};
    return out;
  }
}

impl Projection for ProjectionMutator
{
  // fn get_preclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.preclip;
  // }

  fn preclip(&mut self, preclip: StreamProcessor) {
    // self.preclip = preclip;
    // self.theta = None;
    // return self.reset();
  }

  // fn get_postclip(&self) -> Option<Box<dyn GeoStream>> {
  //   return self.postclip;
  // }

  fn postclip(&mut self, postclip: StreamProcessor) {
    // self.postclip = postclip;
    // self.theta = None;
    // return self.reset();
  }

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

  fn clip_angle(&mut self, angle: StreamProcessorValueMaybe) -> Option<f64>
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

  fn scale(&mut self, scale: Option<&f64>) {
    match scale {
      Some(scale) => {
        self.k = *scale;
        self.recenter();
      }
      None => {}
    }
  }

  fn translate(&mut self, t: Option<&Point>) -> Option<Point> {
    match t {
      Some(t) => {
        self.x = t.x;
        self.y = t.y;
        self.recenter();
        return None;
      }
      None => {
        return Some(Point{x:self.x, y:self.y});
      }
    }
  }

  fn rotate(&mut self, angles: Option<[f64; 3]>) -> Option<[f64; 3]> {
    return match angles {
      Some(angles) => {
        let [delta_lambda, delta_phi, delta_gamma] = angles;
        let f360 = 360f64;
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
