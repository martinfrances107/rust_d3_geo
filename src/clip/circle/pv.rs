use geo::CoordFloat;
use geo::Coordinate;

use crate::clip::PointVisible;

/// Circle PV (Point Visible ).
#[derive(Clone, Debug)]
pub struct PV<T>
where
    T: CoordFloat,
{
    cr: T,
}

impl<T> PV<T>
where
    T: CoordFloat,
{
    /// Given a radius construct the clip circles point visible function.
    pub fn new(radius: T) -> Self {
        Self { cr: radius.cos() }
    }
}

impl<T> PointVisible for PV<T>
where
    T: CoordFloat,
{
    type T = T;
    #[inline]
    fn point_visible(&self, p: &Coordinate<T>, _m: Option<u8>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }
}
