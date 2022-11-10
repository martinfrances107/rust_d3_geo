use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;

use crate::Transform;

/// Combines transforms together.
#[derive(Clone, Copy, Debug)]
pub struct Compose<T, TA, TB> {
    /// Phantom T:
    /// The hidden linkage is if the T in TA is f64
    /// then the T in TB must also be f64.
    p_t: PhantomData<T>,
    pub a: TA,
    pub b: TB,
}

impl<T, TA, TB> Compose<T, TA, TB> {
    #[inline]
    pub const fn new(a: TA, b: TB) -> Self {
        Self {
            p_t: PhantomData::<T>,
            a,
            b,
        }
    }
}

impl<T, TA, TB> Transform for Compose<T, TA, TB>
where
    TA: Transform<T = T>,
    TB: Transform<T = T>,
    T: CoordFloat,
{
    type T = T;
    // Apply A then B.
    fn transform(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.a.transform(coordinate);
        self.b.transform(&temp)
    }

    // Apply B them A.
    fn invert(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        let temp = self.b.invert(coordinate);
        self.a.invert(&temp)
    }
}
