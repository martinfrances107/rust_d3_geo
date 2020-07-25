// Define the default implementation of the trait.
use crate::stream::GeoStream;

/// Define the default implementation of the trait.
pub trait TransformStream<T> {
  fn stream(&self) -> Box<dyn GeoStream<T>>;
  fn point(&mut self, x: T, y: T) { self.stream().point(x, y); }
  fn sphere(&self) { self.stream().sphere(); }
  fn line_start(&mut self) { self.stream().line_start(); }
  fn line_end(&mut self) { self.stream().line_end(); }
  fn polygon_start(&mut self) { self.stream().polygon_start(); }
  fn polygon_end(&mut self) { self.stream().polygon_end(); }
}
