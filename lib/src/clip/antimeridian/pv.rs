use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;

use crate::clip::PointVisible;

/// Antimeridian PV ( Point Visible).
#[derive(Clone, Debug)]
pub struct PV<T> {
    pd: PhantomData<T>,
}

impl<T> Default for PV<T> {
    #[inline]
    fn default() -> PV<T> {
        Self {
            pd: PhantomData::<T>,
        }
    }
}
impl<T> PointVisible for PV<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn point_visible(&self, _p: &Coordinate<T>) -> bool {
        true
    }
}
