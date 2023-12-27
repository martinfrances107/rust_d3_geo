use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::ClipExtentSet;

use super::types::BuilderConicAntimeridianResampleClip;
use super::types::BuilderConicAntimeridianResampleNoClip;

// Code Repeated 2^2 times.
// Variation over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Variation over Resample/None as Resample is rebuilt.
impl<DRAIN, PR, T> ClipExtentSet
    for BuilderConicAntimeridianResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Output = BuilderConicAntimeridianResampleClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<T>; 2]) -> Self::Output {
        Self::Output {
            base: self.base.clip_extent_set(extent),

            phi0: self.phi0,
            phi1: self.phi1,
        }
    }
}
