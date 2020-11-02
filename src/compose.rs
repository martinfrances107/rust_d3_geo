use delaunator::Point;
use std::rc::Rc;

use crate::Transform;

pub struct Compose {
    pub a: Rc<Box<dyn Transform>>,
    pub b: Rc<Box<dyn Transform>>,
}

impl<'a> Compose {
    pub fn new(a: Rc<Box<dyn Transform>>, b: Rc<Box<dyn Transform>>) -> Box<dyn Transform> {
        return Box::new(Self { a, b });
    }
}

impl Transform for Compose {
    // Apply A then B.
    fn transform(&self, coordinates: &Point) -> Point {
        let temp = self.a.transform(coordinates);
        return self.b.transform(&temp);
    }

    // Apply B them A.
    fn invert(&self, coordinates: &Point) -> Point {
        let temp = self.b.invert(coordinates);
        return self.a.invert(&temp);
    }
}
