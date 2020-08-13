use num_traits::Float;
use num_traits::FloatConst;

use std::cell::RefCell;
use std::rc::Rc;

// use crate::stream::Stream;
use crate::transform_stream::TransformStream;
// use crate::transform_stream::TransformStreamIdentity;

pub struct Interpolate<F>
where
  F: Float,
{
  stream: Option<Rc<RefCell<Box<dyn TransformStream<F>>>>>,
}

impl<F> Interpolate<F>
where
  F: Float + FloatConst,
{
  pub fn new() -> Self {
    return Interpolate::<F> { stream: None };
  }
}

impl<F> TransformStream<F> for Interpolate<F>
where
  F: Float + FloatConst,
{
  fn stream(&mut self, stream: &Rc<RefCell<Box<dyn TransformStream<F>>>>) {
    let s = stream.clone();
    self.stream = Some(s);
  }
}

impl<F> Interpolate<F>
where
  F: Float,
{
  fn interpolate(self, from: Option<[F; 2]>, to: [F; 2], direction: F)
  where
    F: Float + FloatConst,
  {
    let phi: F;
    match from {
      None => match self.stream {
        Some(s) => {
          let mut stream = s.borrow_mut();
          phi = direction * F::FRAC_PI_2();
          stream.point(-F::PI(), phi, None);
          stream.point(F::zero(), phi, None);
          stream.point(F::PI(), phi, None);
          stream.point(F::PI(), F::zero(), None);
          stream.point(F::PI(), -phi, None);
          stream.point(F::zero(), -phi, None);
          stream.point(-F::PI(), -phi, None);
          stream.point(-F::PI(), F::zero(), None);
          stream.point(-F::PI(), phi, None);
        }
        None => {}
      },
      Some(from) => match (from[0] - to[0]).abs() > F::epsilon() {
        true => match self.stream {
          Some(s) => {
            let mut stream = s.borrow_mut();
            let lambda: F = if from[0] < to[0] { F::PI() } else { -F::PI() };
            let f_2 = F::from(2u8).unwrap();
            phi = direction * lambda / f_2;
            stream.point(-lambda, phi, None);
            stream.point(F::zero(), phi, None);
            stream.point(lambda, phi, None);
          }
          None => {}
        },
        false => match self.stream {
          Some(s) => {
            let mut stream = s.borrow_mut();
            stream.point(to[0], to[1], None);
          }
          None => {}
        },
      },
    }
  }
}
