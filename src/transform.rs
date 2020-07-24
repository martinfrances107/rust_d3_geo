// Define the default implementation of the trait.
use crate::projection::geo_stream::GeoStream;

/// Define the default implementation of the trait.
pub trait TransformStream {
  fn stream(&self) -> Box<dyn GeoStream>;
  fn point(&mut self, x: f64, y: f64) { self.stream().point(x, y); }
  fn sphere(&self) { self.stream().sphere(); }
  fn line_start(&mut self) { self.stream().line_start(); }
  fn line_end(&mut self) { self.stream().line_end(); }
  fn polygon_start(&mut self) { self.stream().polygon_start(); }
  fn polygon_end(&mut self) { self.stream().polygon_end(); }
}
