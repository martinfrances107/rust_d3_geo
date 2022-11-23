use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::PCNU;
use crate::projection::ClipExtentClear;
use crate::projection::TransformExtent;
use crate::Transform;

use super::Builder;
use super::Reclip;

impl<CLIPC, CLIPU, DRAIN, PR, RC, RU, T> ClipExtentClear
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    RC: Clone,
    RU: Clone,
    T: CoordFloat + FloatConst,
{
    type Output = Self;
    type T = T;

    fn clip_extent_clear(&self) -> Self::Output {
        let mut out = Self::Output {
            base: self.base.clone(),
            pr: self.pr.clone(),
            extent: None,
        };
        out.reclip();
        out
    }
}
