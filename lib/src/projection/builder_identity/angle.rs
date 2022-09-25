use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::AngleSet;

use super::Builder;

impl<DRAIN, PCNU, T> AngleSet for Builder<DRAIN, PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn angle_set(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.sa = self.alpha.sin();
        self.ca = self.alpha.cos();
        self.reset()
    }
}
