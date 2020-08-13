use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use std::cell::RefCell;
use std::rc::Rc;

use crate::cartesian::cartesian;
use crate::math::epsilon;
use crate::transform_stream::TransformStream;
use crate::Transform;

// use crate::stream::Stream;

// import {cartesian} from "../cartesian.js";
// import {transformer} from "../transform.js";

const MAXDEPTH: u8 = 16u8; // maximum depth of subdivision

fn cos_min_distance<F>() -> F
where
  F: Float + FloatConst + FromPrimitive,
{
  return (F::from(30.0f64).unwrap().to_radians()).cos(); // cos(minimum angular distance)
}

// #[derive(Clone)]
pub struct Resample<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  project: Rc<RefCell<Box<dyn Transform<F>>>>,
  delta2: F,

  // first point
  lambda00: F,
  x00: F,
  y00: F,
  a00: F,
  b00: F,
  c00: F,

  // previous point
  lambda0: F,
  x0: F,
  y0: F,
  a0: F,
  b0: F,
  c0: F,

  cos_min_distance: F,
  s: Option<Rc<RefCell<Box<dyn TransformStream<F>>>>>,
  // s: &'a Box<dyn TransformStream<F>>,
  use_line_point: bool,
  use_line_start: bool,
  use_line_end: bool,
}

impl<F> Resample<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  pub fn new(project: Rc<RefCell<Box<dyn Transform<F>>>>, delta2: F) -> Self
  where
    F: Float + FloatConst + FromPrimitive,
  {
    return Self {
      project: project.clone(),
      delta2,

      lambda00: F::zero(),
      x00: F::zero(),
      y00: F::zero(),
      a00: F::zero(),
      b00: F::zero(),
      c00: F::zero(), // first point

      lambda0: F::zero(),
      x0: F::zero(),
      y0: F::zero(),
      a0: F::zero(),
      b0: F::zero(),
      c0: F::zero(),                                                 // previous point
      cos_min_distance: (F::from(30u8).unwrap().to_radians()).cos(), // cos(minimum angular distance)

      // s: &Box::new(TransformStreamIdentity::new()),
      s: None,
      use_line_point: true,
      use_line_end: true,
      use_line_start: true,
    };
  }

  fn ring_start(&mut self) {
    self.line_start();
    self.use_line_point = false;
    self.use_line_end = false;
  }

  fn ring_point(&mut self, lambda: F, phi: F) {
        self.lambda00 = lambda;
        {
          // let self_p: RefCell<_> = RefCell::new(self);
          // let self_p1 = self_p.borrow_mut();
          self.line_point(self.lambda00, phi);
        }
        self.x00 = self.x0;
        self.y00 = self.y0;
        self.a00 = self.a0;
        self.b00 = self.b0;
        self.c00 = self.c0;
        self.use_line_point = true;
  }

  fn ring_end(&mut self) {
    match &self.s {
      Some(s) => {
        let mut stream = s.borrow_mut();
        self.resample_line_to(
          self.x0,
          self.y0,
          self.lambda0,
          self.a0,
          self.b0,
          self.c0,
          self.x00,
          self.y00,
          self.lambda00,
          self.a00,
          self.b00,
          self.c00,
          MAXDEPTH,
          &mut *stream,
        );
        self.use_line_end = true;
      },
      None => {},
    }
    self.line_end();
  }

  fn line_point(&self, lambda: F, phi: F) {
    let c = cartesian(&[lambda, phi]);
    let project = &*self.project.borrow();
    let p = project.transform(&[lambda, phi]);
    // self.resample_line_to(self.x0, self.y0, self.lambda0, self.a0, self.b0, self.c0, self.x0 = p[0], self.y0 = p[1], self.lambda0 = self.lambda, self.a0 = c[0], self.b0 = c[1],self. c0 = c[2], MAXDEPTH, self.stream);
    // stream.point(x0, y0);
  }

  fn resample_line_to(
    &self,
    x0: F,
    y0: F,
    lambda0: F,
    a0: F,
    b0: F,
    c0: F,
    x1: F,
    y1: F,
    lambda1: F,
    a1: F,
    b1: F,
    c1: F,
    depth_p: u8,
    stream: &mut Box<dyn TransformStream<F>>,
  ) where
    F: Float + FloatConst + FromPrimitive,
  {
    let mut depth = depth_p;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let d2 = dx * dx + dy * dy;
    let float_4 = F::from(4u8).unwrap();
    let float_2 = F::from(2u8).unwrap();
    // if (d2 > 4 * delta2 && depth--) {
    if d2 > float_4 * self.delta2 {
      depth = depth - 1u8;
      if depth > 0u8 {
        let mut a = a0 + a1;
        let mut b = b0 + b1;
        let mut c: F = c0 + c1;
        let m: F = (a * a + b * b + c * c).sqrt();
        c = c / m;
        let phi2 = c.asin();
        let lambda2;
        if (c.abs() - F::one()).abs() < epsilon() || (lambda0 - lambda1).abs() < epsilon() {
          lambda2 = (lambda0 + lambda1) / float_2;
        } else {
          lambda2 = b.atan2(a);
        };
        let f_2 = F::from(2u8).unwrap();
        let lambda2 = match (c.abs() - F::one()).abs() < F::epsilon()
          || (lambda0 - lambda1).abs() < F::epsilon()
        {
          true => (lambda0 + lambda1) / f_2,
          false => b.atan2(a),
        };
        let project = &*self.project.borrow();
        let p = project.transform(&[lambda2, phi2]);
        let x2 = p[0];
        let y2 = p[1];
        let dx2 = x2 - x0;
        let dy2 = y2 - y0;
        let dz = dy * dx2 - dx * dy2;
        // Three condtions :-
        // perpendicular projected distance
        // midpoint close to an end
        // angular distance
        // TODO must find a way to make this constants static
        let float_1_2 = F::from(0.5f64).unwrap();
        let float_1_3 = F::from(0.3f64).unwrap();
        if dz * dz / d2 > self.delta2
          || ((dx * dx2 + dy * dy2) / d2 - float_1_2).abs() > float_1_3
          || a0 * a1 + b0 * b1 + c0 * c1 < self.cos_min_distance
        {
          a = a / m;
          b = b / m;
          let stream_p: RefCell<_> = RefCell::new(stream);
          let self_p: RefCell<_> = RefCell::new(self);
          // {
          let mut s = stream_p.borrow_mut();
          let self_p1 = self_p.borrow_mut();
          self_p1.resample_line_to(
            x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b, c, depth, &mut s,
          );
          // }
          // {
          let mut s2 = stream_p.borrow_mut();
          s2.point(x2, y2, None);
          // }
          // {
          let mut s3 = stream_p.borrow_mut();
          let self_p2 = self_p.borrow_mut();
          self_p2.resample_line_to(
            x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth, &mut s3,
          );
          // }
        }
      }
    }
  }
}

impl<F> TransformStream<F> for Resample<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn point(&mut self, x: F, y: F, z: Option<F>) {
    let project = &*self.project.borrow();
    let p = project.transform(&[x, y]);
    self.use_line_point = false;
    // self.stream.point(p[0], p[1]);
  }

  fn line_start(&mut self) {
    match &self.s {
      Some(s) => {
        let mut stream = s.borrow_mut();
        self.x0 = F::nan();
        // resampleStream.point = line_point;
        self.use_line_point = true;
        stream.line_start();
      },
      None => {},
    }
  }

  fn line_end(&mut self) {
    match &self.s {
      Some(s) => {
        let mut stream = s.borrow_mut();
        self.use_line_point = false;
        // resampleStream.point = point;
        stream.line_end();
      },
      None => {},
    }
  }
}
