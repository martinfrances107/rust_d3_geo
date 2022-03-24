use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;

use crate::clip::PointVisible;

/// Antimeridian PV ( Point Visible).
#[derive(Clone, Debug)]
pub struct PV<T>
// where
//     T: AbsDiffEq<Epsilon = T> + CoordFloat,
{
    pd: PhantomData<T>,
}

impl<T> Default for PV<T>
where
    T: AbsDiffEq<Epsilon = T> + CoordFloat,
{
    #[inline]
    fn default() -> PV<T> {
        Self {
            pd: PhantomData::<T>,
        }
    }
}
impl<T> PointVisible for PV<T>
where
    T: AbsDiffEq<Epsilon = T> + CoordFloat,
{
    type T = T;

    #[inline]
    fn point_visible(&self, _p: &Coordinate<T>) -> bool {
        true
    }
}
