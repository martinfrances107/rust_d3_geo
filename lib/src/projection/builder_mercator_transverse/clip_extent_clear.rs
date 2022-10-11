use geo::CoordFloat;

use crate::projection::builder::template::PCNU;
use crate::projection::builder_mercator::Reclip;
use crate::projection::ClipExtentClear;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentClear
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<DRAIN, T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone,
    RC: Clone,
    RU: Clone,
    Self: Reclip,
    T: CoordFloat,
{
    type Output = Self;
    /// f64 or f32.
    type T = T;

    fn clip_extent_clear(&self) -> Self::Output {
        let mut out = Self::Output {
            p_drain: self.p_drain,
            p_rc: self.p_rc,
            p_clipc: self.p_clipc,
            base: self.base.clone(),
            // pr: self.pr.clone(),
            // extent: None,
        };
        out.reclip();
        out
    }
}
