use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::PCNU;
use crate::projection::builder_mercator::Reclip;
use crate::projection::{ClipExtentClear, TransformExtent};
use crate::Transform;

use super::Builder;

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
    /// f64 or f32.
    type T = T;

    fn clip_extent_clear(&self) -> Self::Output {
        let mut out = Self::Output {
            base: self.base.clone(),
        };
        out.reclip();
        out
    }
}
