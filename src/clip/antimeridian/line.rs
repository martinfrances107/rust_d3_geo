use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::stream::GeoStream;

use super::intersect::intersect;

// import clip from "./index.js";

// export default clip(
//   function() { return true; },
//   clipAntimeridianLine,
//   clipAntimeridianInterpolate,
//   [-PI, -halfPI]
// );

// Return indicator :-
// There were intersections or the line was empty.
const INTERSECTION_OR_LINE_EMPTY: u8 = 0u8;
const NO_INTERSECTIONS: u8 = 1u8;
// There were intersectoins and the first and last sections should be rejoined.
const INTERSECTION_REJOIN: u8 = 2u8;

use crate::clip::ClipLineTrait;


pub struct ClipAntimeridianLine<F> {
  clean: Option<u8>,
  lambda0: F,
  phi0: F,
  sign0: F,
  stream: Box<dyn ClipLineTrait<F>>,
}

impl<F> ClipAntimeridianLine<F>
where
  F: Float,
{
  pub fn new(stream: Box<dyn ClipLineTrait<F>>) -> Self {
    return Self {
      clean: None, // no intersections
      lambda0: F::nan(),
      phi0: F::nan(),
      sign0: F::nan(),
      stream,
    };
  }
}

impl<F> ClipLineTrait<F> for ClipAntimeridianLine<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn line_start(&mut self) {
    self.stream.line_start();
    self.clean = Some(NO_INTERSECTIONS);
  }

  fn point(&mut self, lambda1: F, phi1: F) {
    let sign1 = match lambda1 > F::zero() {
      true => F::PI(),
      false => -F::PI(),
    };
    let delta = (lambda1 - self.lambda0).abs();

    if (delta - F::PI()).abs() < F::epsilon() {
      // Line crosses a pole.
      let f_2 = F::from(2u8).unwrap();
      self.phi0 = (self.phi0 + phi1) / f_2;
      match (self.phi0 + phi1 / f_2) > F::zero() {
        true => {
          self.stream.point(self.lambda0, F::FRAC_PI_2());
        }
        false => {
          self.stream.point(self.lambda0, -F::FRAC_PI_2());
        }
      }
      self.stream.point(self.sign0, self.phi0);
      self.stream.line_end();
      self.stream.line_start();
      self.stream.point(sign1, self.phi0);
      self.stream.point(lambda1, self.phi0);
      self.clean = Some(INTERSECTION_OR_LINE_EMPTY);
    } else if self.sign0 != sign1 && delta >= F::PI() {
      // Line crosses antimeridian.
      if (self.lambda0 - self.sign0).abs() < F::epsilon() {
        self.lambda0 = self.lambda0 - self.sign0 * F::epsilon(); // handle degeneracies
      }
      if (lambda1 - sign1).abs() < F::epsilon() {
        lambda1 = lambda1 - sign1 * F::epsilon();
      }
      self.phi0 = intersect(self.lambda0, self.phi0, lambda1, phi1);
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
    self.lambda0 = F::nan();
    self.phi0 = F::nan();
  }

  fn clean(&mut self) -> Option<u8> {
    return match self.clean {
      Some(clean) => Some(2u8 - clean), // if intersections, rejoin first and last segments
      None => None,
    };
  }
}
