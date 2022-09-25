use geo::CoordFloat;
use geo::Coordinate;

use crate::projection::TranslateSet;

use super::Builder;

impl<DRAIN, PCNU, T> TranslateSet for Builder<DRAIN, PCNU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate_set(mut self, t: &Coordinate<T>) -> Self {
        self.tx = t.x;
        self.ty = t.y;
        self
    }
}
