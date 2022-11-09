use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::{ReflectSet, REFLECT};

use super::Builder;

impl<DRAIN, PCNU, T> ReflectSet for Builder<DRAIN, PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sx = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.reset()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sy = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.reset()
    }
}
