use num_traits::Float;
use num_traits::FloatConst;

pub trait GeoStream<F>
where F: Float {
  fn point(&mut self, x: F, y: F);
  fn sphere(&mut self);
  fn line_start(&mut self);
  fn line_end(&mut self);
  fn polygon_start(&mut self);
  fn polygon_end(&mut self);
}
