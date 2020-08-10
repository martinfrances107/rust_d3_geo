use num_traits::Float;

use crate::Transform;

pub struct Compose<F> {
  pub a: Box<dyn Transform<F>>,
  pub b: Box<dyn Transform<F>>,
}

impl<'a, F> Compose<F> {
  pub fn new(a: Box<dyn Transform<F>>, b: Box<dyn Transform<F>>) -> Self {
    return Self { a, b };
  }
}

impl<F> Transform<F> for Compose<F>
where F: Float {
  // Apply A then B.
  fn transform(&self, coordinates: &[F; 2]) -> [F; 2] {
    let temp = self.a.transform(coordinates);
    return self.b.transform(&temp);
  }

  // Apply B them A.
  fn invert(&self, coordinates: &[F; 2]) -> [F; 2] {
    let temp = self.b.invert(coordinates);
    return self.a.invert(&temp);
  }
}
