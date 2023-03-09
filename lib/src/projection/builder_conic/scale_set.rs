use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;

use crate::projection::ScaleSet;

impl<BASE, T> ScaleSet for Builder<BASE, T>
where
    BASE: ScaleSet<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.base.scale_set(scale);
        self
    }
}
