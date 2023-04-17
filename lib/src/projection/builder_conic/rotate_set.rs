use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::RotateSet;
use crate::projection::ScaleGet;

use super::Builder;

impl<BASE, T> RotateSet for Builder<BASE, T>
where
    BASE: RotateSet<T = T> + ScaleGet<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn rotate2_set(&mut self, angles: &[T; 2]) -> &mut Self {
        self.base.rotate2_set(angles);
        self
    }

    fn rotate3_set(&mut self, angles: &[T; 3]) -> &mut Self {
        self.base.rotate3_set(angles);
        self
    }
}
