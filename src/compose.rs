use crate::Transform;
use crate::TransformClone;
use geo::{CoordFloat, Coordinate};

pub struct Compose<T>
where
    T: CoordFloat,
{
    pub a: Box<dyn Transform<TcC = Coordinate<T>>>,
    pub b: Box<dyn Transform<TcC = Coordinate<T>>>,
}

impl<T: CoordFloat + 'static> Clone for Compose<T> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone_box(),
            b: self.b.clone_box(),
        }
    }
}

impl<T: CoordFloat + 'static> TransformClone for Compose<T> {
    type TcC = Coordinate<T>;
    fn clone_box(&self) -> Box<dyn Transform<TcC = Self::TcC>> {
        Box::new(Self {
            a: self.a.clone_box(),
            b: self.b.clone_box(),
        })
    }
}
impl<'a, T: CoordFloat + 'static> Compose<T> {
    #[inline]
    pub fn new(
        a: Box<dyn Transform<TcC = Coordinate<T>>>,
        b: Box<dyn Transform<TcC = Coordinate<T>>>,
    ) -> Box<dyn Transform<TcC = Coordinate<T>>> {
        Box::new(Self {
            a: a.clone_box(),
            b: b.clone_box(),
        })
    }
}

impl<T: CoordFloat + 'static> Transform for Compose<T> {
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
