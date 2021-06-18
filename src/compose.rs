use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::Transform;

#[derive(Debug, Default)]
pub struct Compose<T, TA, TB>
where
    T: CoordFloat + Default + FloatConst,
    TA: Transform<C = Coordinate<T>>,
    TB: Transform<C = Coordinate<T>>,
{
    pub a: TA,
    pub b: TB,
}

impl<T, TA, TB> Compose<T, TA, TB>
where
    T: CoordFloat + Default + FloatConst,
    TA: Transform<C = Coordinate<T>>,
    TB: Transform<C = Coordinate<T>>,
{
    #[inline]
    pub fn new(a: TA, b: TB) -> Compose<T, TA, TB> {
        Compose { a: a, b: b }
    }
}

impl<T, TA, TB> Transform for Compose<T, TA, TB>
where
    TA: Transform<C = Coordinate<T>>,
    TB: Transform<C = Coordinate<T>>,
    T: CoordFloat + Default + FloatConst,
{
    type C = Coordinate<T>;
    // Apply A then B.
    fn transform(&self, coordinates: &<TA as Transform>::C) -> Coordinate<T> {
        let temp = self.a.transform(coordinates);
        self.b.transform(&temp)
    }

    // Apply B them A.
    fn invert(&self, coordinates: &<TA as Transform>::C) -> Coordinate<T> {
        let temp = self.b.invert(coordinates);
        self.a.invert(&temp)
    }
}
