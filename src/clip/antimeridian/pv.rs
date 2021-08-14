use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

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
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;

    #[inline]
    fn point_visible(&self, _p: &Coordinate<T>, _z: Option<u8>) -> bool {
        true
    }
}
