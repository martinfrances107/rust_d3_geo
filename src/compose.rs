use crate::Transform;
use geo::{CoordFloat, Coordinate};
use std::rc::Rc;

#[derive(Clone)]
pub struct Compose<T>
where
    T: CoordFloat,
{
    pub a: Rc<Box<dyn Transform<C = Coordinate<T>>>>,
    pub b: Rc<Box<dyn Transform<C = Coordinate<T>>>>,
}

impl<'a, T: CoordFloat + 'static> Compose<T> {
    #[inline]
    pub fn new(
        a: Rc<Box<dyn Transform<C = Coordinate<T>>>>,
        b: Rc<Box<dyn Transform<C = Coordinate<T>>>>,
    ) -> Box<dyn Transform<C = Coordinate<T>>> {
        Box::new(Self {
            a: a.clone(),
            b: b.clone(),
        })
    }
}

impl<T: CoordFloat> Transform for Compose<T> {
    type C = Coordinate<T>;
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
