use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::Transform;

#[derive(Clone, Debug, Default)]
pub struct Compose<T, TA, TB>
where
    T: CoordFloat + Default + FloatConst,
    TA: Transform<TcC = Coordinate<T>> + Clone,
    TB: Transform<TcC = Coordinate<T>> + Clone,
{
    pub a: TA,
    pub b: TB,
}

impl<T, TA, TB> Compose<T, TA, TB>
where
    T: CoordFloat + Default + FloatConst,
    TA: Transform<TcC = Coordinate<T>> + Clone,
    TB: Transform<TcC = Coordinate<T>> + Clone,
{
    #[inline]
    pub fn new(a: TA, b: TB) -> Compose<T, TA, TB> {
        Compose { a: a, b: b }
    }
}

impl<T, TA, TB> Transform for Compose<T, TA, TB>
where
    TA: Transform<TcC = Coordinate<T>> + Clone,
    TB: Transform<TcC = Coordinate<T>> + Clone,
    T: CoordFloat + Default + FloatConst,
{
    type TcC = Coordinate<T>;
    // Apply A then B.
    fn transform(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.a.transform(coordinates);
        self.b.transform(&temp)
    }

    // Apply B them A.
    fn invert(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.b.invert(coordinates);
        self.a.invert(&temp)
    }
}
