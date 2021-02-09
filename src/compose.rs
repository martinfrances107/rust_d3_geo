use crate::Transform;
use geo::{CoordFloat, Coordinate};
use std::rc::Rc;

#[derive(Clone)]
pub struct Compose<T> {
    pub a: Rc<Box<dyn Transform<T>>>,
    pub b: Rc<Box<dyn Transform<T>>>,
}

impl<'a, T: CoordFloat + 'static> Compose<T> {
    #[inline]
    pub fn new(
        a: Rc<Box<dyn Transform<T>>>,
        b: Rc<Box<dyn Transform<T>>>,
    ) -> Box<dyn Transform<T>> {
        Box::new(Self {
            a: a.clone(),
            b: b.clone(),
        })
    }
}

impl<T: CoordFloat> Transform<T> for Compose<T> {
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
