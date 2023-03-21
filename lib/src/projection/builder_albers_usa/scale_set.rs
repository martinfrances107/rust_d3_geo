use super::Builder;

use crate::projection::ScaleSet;
use crate::projection::TranslateGet;
use crate::projection::TranslateSet;

impl<DRAIN> ScaleSet for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type T = f64;

    fn scale_set(&mut self, scale: f64) -> &mut Self {
        self.pr.alaska.scale_set(0.35_f64 * scale);
        self.pr.lower_48.scale_set(scale);
        self.pr.hawaii.scale_set(scale);
        self.translate_set(&self.pr.lower_48.translate())
    }
}
