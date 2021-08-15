use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::PointVisible;

/// Circle PV (Point Visible ).
#[derive(Clone, Debug)]
pub struct PV<T> {
    cr: T,
}

impl<T> PV<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(radius: T) -> Self {
        Self { cr: radius.cos() }
    }
}
impl<T> PointVisible for PV<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
    #[inline]
    fn point_visible(&self, p: &Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }
}
