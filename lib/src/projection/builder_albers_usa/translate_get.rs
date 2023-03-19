use geo_types::Coord;

use crate::projection::TranslateGet;

use super::Builder;

impl<DRAIN> TranslateGet for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type T = f64;

    #[inline]
    fn translate(&self) -> Coord<f64> {
        self.pr.lower_48.translate()
    }
}
