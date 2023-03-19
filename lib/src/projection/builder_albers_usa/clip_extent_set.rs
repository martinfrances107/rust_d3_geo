use geo_types::Coord;

use crate::projection::ClipExtentSet;

use super::Builder;

// Code Repeated 2^2 times.
// Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Varariantion over Resample/None as Resample is rebuilt.
impl<DRAIN> ClipExtentSet for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type Output = Builder<DRAIN>;
    type T = f64;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<f64>; 2]) -> Self::Output {
        let mut out = self.clone();
        out.pr.alaska_point = self.pr.alaska.clip_extent_set(extent);
        out
    }
}
