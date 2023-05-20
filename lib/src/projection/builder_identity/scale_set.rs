use geo::CoordFloat;

use crate::projection::ScaleSet;

use super::Builder;

impl<PCNU, T> ScaleSet for Builder<PCNU, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn scale_set(&mut self, k: T) -> &mut Self {
        self.k = k;
        self.reset()
    }
}
