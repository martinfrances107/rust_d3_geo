use num_traits::Float;
use std::rc::Rc;

use crate::Transform;

pub struct Compose<F> {
  pub a: Rc<Box<dyn Transform<F>>>,
  pub b: Rc<Box<dyn Transform<F>>>,
}

impl<'a, F> Compose<F>
where
  F: Float + 'static,
{
  pub fn new(a: Rc<Box<dyn Transform<F>>>, b: Rc<Box<dyn Transform<F>>>) -> Box<dyn Transform<F>> {
    return Box::new(Self { a, b });
  }
}

impl<F> Transform<F> for Compose<F>
where
  F: Float,
{
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
