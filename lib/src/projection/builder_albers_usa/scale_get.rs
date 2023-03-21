use super::Builder;

use crate::projection::ScaleGet;

impl<DRAIN> ScaleGet for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type T = f64;

    fn scale(&self) -> Self::T {
        self.pr.lower_48.scale()
    }
}
