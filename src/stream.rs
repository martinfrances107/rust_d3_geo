use num_traits::Float;

pub trait Stream<F>
where F: Float {
  #[allow(unused_variables)]
  fn point(&mut self, x: F, y: F, z: Option<F>) {}
  fn sphere(&mut self) {}
  fn line_start(&mut self) {}
  fn line_end(&mut self) {}
  fn polygon_start(&mut self) {}
  fn polygon_end(&mut self) {}
}
