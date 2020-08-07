use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

// use crate::stream::GeoStream;
use crate::transform_stream::TransformStream;
use crate::transform_stream::TransformStreamIdentity;

use super::intersect::intersect;

// Return indicator :-
// There were intersections or the line was empty.
const INTERSECTION_OR_LINE_EMPTY: u8 = 0u8;
const NO_INTERSECTIONS: u8 = 1u8;
// There were intersectoins and the first and last sections should be rejoined.
const INTERSECTION_REJOIN: u8 = 2u8;

// use crate::clip::ClipLine;

pub struct ClipAntimeridianLine<F> {
  clean: Option<u8>,
  lambda0: F,
  phi0: F,
  sign0: F,
  stream: Box<dyn TransformStream<F>>,
}

impl<'a, F> ClipAntimeridianLine<F>
where
  F: Float,
{
  pub fn new() -> Self {
    return ClipAntimeridianLine::<F> {
      clean: None, // no intersections
      lambda0: F::nan(),
      phi0: F::nan(),
      sign0: F::nan(),
      stream: Box::new(TransformStreamIdentity::new::<F>()),
    };
  }

  fn clean(&mut self) -> Option<u8> {
    return match self.clean {
      Some(clean) => Some(2u8 - clean), // if intersections, rejoin first and last segments
      None => None,
    };
  }
}

impl<F> TransformStream<F> for ClipAntimeridianLine<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  fn stream(&mut self, stream: Box<dyn TransformStream<F>>) {
    self.stream = stream;
  }

  fn line_start(&mut self) {
    self.stream.line_start();
    self.clean = Some(NO_INTERSECTIONS);
  }

  fn point(&mut self, mut lambda1: F, phi1: F, _m: Option<F>) {
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
          self.stream.point(self.lambda0, F::FRAC_PI_2(), None);
        }
        false => {
          self.stream.point(self.lambda0, -F::FRAC_PI_2(), None);
        }
      }
      self.stream.point(self.sign0, self.phi0, None);
      self.stream.line_end();
      self.stream.line_start();
      self.stream.point(sign1, self.phi0, None);
      self.stream.point(lambda1, self.phi0, None);
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
      self.stream.point(self.sign0, self.phi0, None);
      self.stream.line_end();
      //  self.stream.line_start();
      self.stream.point(sign1, self.phi0, None);
      self.clean = Some(INTERSECTION_OR_LINE_EMPTY);
    }
    self.lambda0 = lambda1;
    self.phi0 = phi1;
    self.stream.point(self.lambda0, self.phi0, None);
    self.sign0 = sign1;
  }

  fn line_end(&mut self) {
    self.stream.line_end();
    self.lambda0 = F::nan();
    self.phi0 = F::nan();
  }
}
