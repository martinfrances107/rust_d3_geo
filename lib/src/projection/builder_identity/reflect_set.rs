use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::{Reflect, ReflectSet};

use super::Builder;

impl<PCNU, T> ReflectSet for Builder<PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sx = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.reset()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sy = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.reset()
    }
}
