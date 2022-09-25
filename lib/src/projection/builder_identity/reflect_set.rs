use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ReflectSet;

use super::Builder;

impl<DRAIN, PCNU, T> ReflectSet for Builder<DRAIN, PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sy = T::from(-1.0_f64).unwrap();
        } else {
            self.sy = T::one();
        }
        self
    }
}
