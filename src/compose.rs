use geo::Coordinate;
use num_traits::Float;

use std::rc::Rc;

use crate::Transform;

pub struct Compose<T> {
    pub a: Rc<Box<dyn Transform<T>>>,
    pub b: Rc<Box<dyn Transform<T>>>,
}

impl<'a, T: Float + 'static> Compose<T> {
    pub fn new(
        a: Rc<Box<dyn Transform<T>>>,
        b: Rc<Box<dyn Transform<T>>>,
    ) -> Box<dyn Transform<T>> {
        return Box::new(Self { a, b });
    }
}

impl<T: Float> Transform<T> for Compose<T> {
    // Apply A then B.
    fn transform(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.a.transform(coordinates);
        return self.b.transform(&temp);
    }

    // Apply B them A.
    fn invert(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.b.invert(coordinates);
        return self.a.invert(&temp);
    }
}
