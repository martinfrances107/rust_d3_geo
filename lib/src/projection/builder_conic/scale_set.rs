use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use super::PRConic;

use crate::projection::ScaleSet;
use crate::Transform;

impl<BASE, PR, T> ScaleSet for Builder<BASE, PR, T>
where
    BASE: ScaleSet<T = T>,
    PR: Clone + PRConic + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale_set(&mut self, scale: T) -> &mut Self {
        self.base.scale_set(scale);
        self
    }
}
