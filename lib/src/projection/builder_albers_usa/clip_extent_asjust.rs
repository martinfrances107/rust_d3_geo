use geo_types::Coord;

use crate::projection::ClipExtentAdjust;
use crate::projection::ClipExtentSet;

use super::Builder;

// Code Repeated 2^2 times.
// Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Varariantion over Resample/None as Resample is rebuilt.
impl<DRAIN> ClipExtentAdjust for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type T = f64;

    #[inline]
    fn clip_extent_adjust(&mut self, extent: &[Coord<f64>; 2]) -> &mut Self {
        self.pr.alaska.clip_extent_set(extent);
        self
    }
}
