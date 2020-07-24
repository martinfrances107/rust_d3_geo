use std::f64::consts::PI;
use std::f64::NAN;

use crate::math::HALFPI;
use crate::math::EPSILON;

use crate::projection::geo_stream::GeoStream;
use super::antimeridian_intersect::antimeridian_intersect;

// import clip from "./index.js";

// export default clip(
//   function() { return true; },
//   clipAntimeridianLine,
//   clipAntimeridianInterpolate,
//   [-PI, -halfPI]
// );


// Return indicator :-
// There were intersections or the line was empty.
const INTERSECTION_OR_LINE_EMPTY:u8 = 0u8;
const NO_INTERSECTIONS:u8 = 1u8;
// There were intersectoins and the first and last sections should be rejoined.
const INTERSECTION_REJOIN:u8 = 2u8;

pub trait ClipAntimeridian {
  fn line_start(&mut self);
  fn point(&mut self, p:[f64;2]);
  fn line_end(&mut self);
  fn clean(&mut self) -> Option<u8>;
}

pub struct ClipAntimeridianState{
  lambda0:f64,
  phi0:f64,
  sign0:f64,
  clean: Option<u8>,
  stream: Box<dyn GeoStream>,
}

impl ClipAntimeridianState {
  fn new(stream : Box<dyn GeoStream>) -> Self {
    return Self {
      stream,
      lambda0: f64::NAN,
      phi0: f64::NAN,
      sign0: f64::NAN,
      clean: None, // no intersections
   };
  }
}

impl ClipAntimeridian for ClipAntimeridianState{

fn line_start(&mut self) {
  self.stream.line_start();
  self.clean = Some(NO_INTERSECTIONS);
}

fn point(&mut self, p:[f64;2]) {
  let mut lambda1 = p[0];
  let phi1 = p[1];
  let sign1 = match lambda1 > 0f64 {
      true => PI,
      false => -PI
  };
  let delta = (lambda1 - self.lambda0).abs();

  if (delta - PI).abs() < EPSILON {
    // Line crosses a pole.
    self.phi0 = (self.phi0 + phi1) / 2f64;
    match (self.phi0 + phi1 / 2f64) > 0f64{
      true  => {self.stream.point(self.lambda0,  HALFPI); },
      false => {self.stream.point(self.lambda0, -HALFPI); }
    }
    self.stream.point(self.sign0, self.phi0);
    self.stream.line_end();
    self.stream.line_start();
    self.stream.point(sign1, self.phi0);
    self.stream.point(lambda1, self.phi0);
    self.clean = Some(INTERSECTION_OR_LINE_EMPTY);

  } else if self.sign0 != sign1 && delta >= PI {
  // Line crosses antimeridian.
  if (self.lambda0 - self.sign0).abs() < EPSILON {
    self.lambda0 -= self.sign0 * EPSILON;  // handle degeneracies
  }
  if (lambda1 - sign1).abs() < EPSILON {
    lambda1 -= sign1 * EPSILON;
  }
   self.phi0 = antimeridian_intersect(self.lambda0, self.phi0, lambda1, phi1);
   self.stream.point(self.sign0, self.phi0);
   self.stream.line_end();
  //  self.stream.line_start();
   self.stream.point(sign1, self.phi0);
   self.clean = Some(INTERSECTION_OR_LINE_EMPTY);
 }
 self.lambda0 = lambda1;
 self.phi0 = phi1;
 self.stream.point(self.lambda0, self.phi0);
 self.sign0 = sign1;
}

fn line_end(&mut self) {
  self.stream.line_end();
  self.lambda0 = NAN;
  self.phi0 = NAN;
}

fn clean(&mut self) -> Option<u8> {
  return match self.clean{
    Some(clean) =>  Some(2u8 - clean), // if intersections, rejoin first and last segments
    None => None,
  };
}

}



