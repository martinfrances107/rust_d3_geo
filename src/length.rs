use num_traits::Float;

use super::data_object::DataObject;
use super::stream::convert_obj_to_stream::convert_obj_to_stream;
use super::stream::Stream;

enum PointState {
  Noop,
  First,
  Point,
}
pub struct LengthStream<F>
where
  F: Float,
{
  length_sum: F,
  lambda0: F,
  sin_phi0: F,
  cos_phi0: F,
  point_state: PointState,
  use_length_line_end: bool,
}

impl<F> Default for LengthStream<F>
where
  F: Float,
{
  fn default() -> Self {
    return Self {
      length_sum: F::zero(),
      lambda0: F::zero(),
      sin_phi0: F::zero(),
      cos_phi0: F::zero(),
      point_state: PointState::Noop,
      use_length_line_end: false,
    };
  }
}

impl<F> LengthStream<F>
where
  F: Float,
{
  pub fn calc(object: DataObject<F>) -> F {
    let mut ls = LengthStream::default();
    convert_obj_to_stream(&object, &mut ls);
    return ls.length_sum;
  }

  fn length_point_first(&mut self, lambda_p: F, phi_p: F) {
    let lambda = lambda_p.to_radians();
    let phi = phi_p.to_radians();
    self.lambda0 = lambda;
    self.sin_phi0 = phi.sin();
    self.cos_phi0 = phi.cos();
    self.point_state = PointState::Point;
  }

  fn length_point(&mut self, lambda_p: F, phi_p: F) {
    let lambda = lambda_p.to_radians();
    let phi = phi_p.to_radians();

    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let delta = (lambda - self.lambda0).abs();
    let cos_delta = (delta).cos();
    let sin_delta = (delta).sin();

    let x = cos_phi * sin_delta;
    let y = self.cos_phi0 * sin_phi - self.sin_phi0 * cos_phi * cos_delta;
    let z = self.sin_phi0 * sin_phi + self.cos_phi0 * cos_phi * cos_delta;

    self.length_sum = self.length_sum + ((x * x + y * y).sqrt()).atan2(z);
    self.lambda0 = lambda;
    self.sin_phi0 = sin_phi;
    self.cos_phi0 = cos_phi;
  }
}

impl<F> Stream<F> for LengthStream<F>
where
  F: Float,
{
  fn sphere(&mut self) {}

  fn point(&mut self, x: F, y: F, _z: Option<F>) {
    println!("enter point");
    match self.point_state {
      PointState::Noop => {
        // Do nothing.
      }
      PointState::First => {
        self.length_point_first(x, y);
      }
      PointState::Point => {
        self.length_point(x, y);
      }
    }
  }

  fn line_start(&mut self) {
    self.point_state = PointState::First;
    self.use_length_line_end = true;
  }

  fn line_end(&mut self) {
    if self.use_length_line_end {
      self.point_state = PointState::Noop;
      self.use_length_line_end = false;
    }
  }
  fn polygon_start(&mut self) {}

  fn polygon_end(&mut self) {}
}
