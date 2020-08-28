pub mod convert_obj_to_stream;
mod geometry_processor;
mod line;
mod polygon;

use num_traits::Float;

pub trait Stream<F>
where
  F: Float,
{
  fn point(&mut self, _x: F, _y: F, _z: Option<F>) {}
  fn sphere(&mut self) {}
  fn line_start(&mut self) {}
  fn line_end(&mut self) {}
  fn polygon_start(&mut self) {}
  fn polygon_end(&mut self) {}
}
