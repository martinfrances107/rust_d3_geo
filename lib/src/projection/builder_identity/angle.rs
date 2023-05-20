use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::AngleSet;

use super::Builder;

impl<PCNU, T> AngleSet for Builder<PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn angle_set(&mut self, angle: T) -> &mut Self {
        self.alpha = (angle % self.t360).to_radians();
        let (sa, ca) = self.alpha.sin_cos();
        self.sa = sa;
        self.ca = ca;
        self.reset()
    }
}
