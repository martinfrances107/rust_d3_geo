pub trait GeoStream {
  fn point(&mut self, x: f64, y: f64);
  fn sphere(&mut self);
  fn line_start(&mut self);
  fn line_end(&mut self);
  fn polygon_start(&mut self);
  fn polygon_end(&mut self);
}

// struct StreamWrapper{}

// impl GeoStream for StreamWrapper{}
