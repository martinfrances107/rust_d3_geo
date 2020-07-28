use num_traits::Float;
use num_traits::FloatConst;

// Define the default implementation of the trait.
use crate::stream::GeoStream;

/// Define the default implementation of the trait.
pub trait TransformStream<F>
where F: Float {
  fn stream(&self) -> Box<dyn GeoStream<F>>;
  fn point(&mut self, x: F, y: F) { self.stream().point(x, y); }
  fn sphere(&self) { self.stream().sphere(); }
  fn line_start(&mut self) { self.stream().line_start(); }
  fn line_end(&mut self) { self.stream().line_end(); }
  fn polygon_start(&mut self) { self.stream().polygon_start(); }
  fn polygon_end(&mut self) { self.stream().polygon_end(); }
}
