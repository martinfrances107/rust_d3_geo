use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

/// Combines transform together.
#[derive(Clone, Copy, Debug)]
pub struct Compose<T, TA, TB>
where
    T: CoordFloat,
    TA: Transform<T = T>,
    TB: Transform<T = T>,
{
    pub a: TA,
    pub b: TB,
}

impl<T, TA, TB> Compose<T, TA, TB>
where
    T: CoordFloat,
    TA: Transform<T = T>,
    TB: Transform<T = T>,
{
    #[inline]
    pub fn new(a: TA, b: TB) -> Compose<T, TA, TB> {
        Compose { a, b }
    }
}

impl<T, TA, TB> Transform for Compose<T, TA, TB>
where
    TA: Transform<T = T>,
    TB: Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;
    // Apply A then B.
    fn transform(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.a.transform(coordinate);
        let b = self.b.transform(&temp);
        println!("compose {:?} {:?} - {:?} {:?}", temp.x, temp.y, b.x, b.y);
        b
    }

    // Apply B them A.
    fn invert(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.b.invert(coordinate);
        self.a.invert(&temp)
    }
}
