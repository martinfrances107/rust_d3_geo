pub mod convert_obj_to_stream;
mod geometry_processor;
mod line;
mod polygon;

use delaunator::Point;

pub trait Stream
{
  fn point(&mut self, _x: f64, _y: f64, _z: Option<f64>) {}
  fn sphere(&mut self) {}
  fn line_start(&mut self) {}
  fn line_end(&mut self) {}
  fn polygon_start(&mut self) {}
  fn polygon_end(&mut self) {}
}
