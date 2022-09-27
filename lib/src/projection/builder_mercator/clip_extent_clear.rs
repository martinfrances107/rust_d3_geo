use geo::CoordFloat;

use crate::projection::builder::template::ClipU;
use crate::projection::ClipExtentClear;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentClear
    for Builder<CLIPC, CLIPU, DRAIN, ClipU<DRAIN, T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
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
