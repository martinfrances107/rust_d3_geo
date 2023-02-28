use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::TranslateSet;
use crate::Transform;

use super::Builder;

impl<BASE, PR, T> TranslateSet for Builder<BASE, PR, T>
where
    BASE: TranslateSet<T = T>,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.base.translate_set(t);
        self
    }
}
