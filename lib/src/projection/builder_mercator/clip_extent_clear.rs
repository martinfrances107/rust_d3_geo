use geo::CoordFloat;

use crate::projection::builder::template::ClipU;
use crate::projection::ClipExtentClear;

use super::Builder;
use super::Reclip;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentClear
    for Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
    Self: Reclip,
    T: CoordFloat,
{
    type Output = Self;
    /// f64 or f32.
    type T = T;

    fn clip_extent_clear(mut self) -> Self {
        self.extent = None;
        self.reclip()
    }
}
