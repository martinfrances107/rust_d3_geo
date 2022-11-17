use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::TranslateSet;

use super::Builder;

impl<DRAIN, PCNU, T> TranslateSet for Builder<DRAIN, PCNU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.tx = t.x;
        self.ty = t.y;
        self.reset()
    }
}
