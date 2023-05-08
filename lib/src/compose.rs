use geo::CoordFloat;
use geo_types::Coord;

use crate::Transform;

/// Combines transforms together.
#[derive(Clone, Debug)]
pub struct Compose<TA, TB> {
    pub a: TA,
    pub b: TB,
}

impl<TA, TB> Compose<TA, TB> {
    #[inline]
    pub const fn new(a: TA, b: TB) -> Self {
        Self { a, b }
    }
}

impl<T, TA, TB> Transform for Compose<TA, TB>
where
    T: CoordFloat,
    TA: Transform<T = T>,
    TB: Transform<T = T>,
{
    type T = T;
    // Apply A then B.
    fn transform(&self, coordinate: &Coord<T>) -> Coord<T> {
        let temp = self.a.transform(coordinate);
        self.b.transform(&temp)
    }

    // Apply B them A.
    fn invert(&self, coordinate: &Coord<T>) -> Coord<T> {
        let temp = self.b.invert(coordinate);
        self.a.invert(&temp)
    }
}
