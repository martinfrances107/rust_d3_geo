use num_traits::Float;
use num_traits::FloatConst;

// use crate::stream::Stream;
use crate::transform_stream::TransformStream;
use crate::transform_stream::TransformStreamIdentity;

pub struct Interpolate<F>
where F: Float {
  stream: Box<dyn TransformStream<F>>,
}

impl <F> Interpolate<F>
where F: Float + FloatConst {
  pub fn new() -> Self
  {
    return
      Interpolate::<F>{stream: Box::new(TransformStreamIdentity::new::<F>())};
  }
}

impl<F> TransformStream<F> for Interpolate<F>
where
  F: Float + FloatConst,
{
  fn stream(&mut self, stream: Box<dyn TransformStream<F>>) {
    self.stream = stream;
  }
}

impl<F> Interpolate<F>
where F: Float {
  fn interpolate(mut self, from: Option<[F; 2]>, to: [F; 2], direction: F)
  where
    F: Float + FloatConst,
  {
    let phi: F;
    match from {
      None => {
        phi = direction * F::FRAC_PI_2();
        self.stream.point(-F::PI(), phi, None);
        self.stream.point(F::zero(), phi, None);
        self.stream.point(F::PI(), phi, None);
        self.stream.point(F::PI(), F::zero(), None);
        self.stream.point(F::PI(), -phi, None);
        self.stream.point(F::zero(), -phi, None);
        self.stream.point(-F::PI(), -phi, None);
        self.stream.point(-F::PI(), F::zero(), None);
        self.stream.point(-F::PI(), phi, None);
      },
      Some(from) => match (from[0] - to[0]).abs() > F::epsilon() {
        true => {
          let lambda: F = if from[0] < to[0] { F::PI() } else { -F::PI() };
          let f_2 = F::from(2u8).unwrap();
          phi = direction * lambda / f_2;
          self.stream.point(-lambda, phi, None);
          self.stream.point(F::zero(), phi, None);
          self.stream.point(lambda, phi, None);
        }
        false => {
          self.stream.point(to[0], to[1], None);
        }
      }
    }
  }
}
