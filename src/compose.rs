use crate::Transform;

pub struct Compose<T> {
  pub a: Box<dyn Transform<T>>,
  pub b: Box<dyn Transform<T>>,
}

impl<T> Transform<T> for Compose<T> {
  // Apply A then B.
  fn transform(&self, coordinates: &[T; 2]) -> [T; 2] {
    let temp = self.a.transform(coordinates);
    return self.b.transform(&temp);
  }

  // Apply B them A.
  fn invert(&self, coordinates: &[T; 2]) -> [T; 2] {
    let temp = self.b.invert(coordinates);
    return self.a.invert(&temp);
  }
}
