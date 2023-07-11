use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ClipAngleSet;

use super::types::BuilderConicAntimeridianResampleNoClip;
use super::types::BuilderConicCircleResampleNoClip;

impl<DRAIN, PR, T> ClipAngleSet for BuilderConicAntimeridianResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type Output = BuilderConicCircleResampleNoClip<DRAIN, PR, T>;
    /// f32 or f64.
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle strategy.
    #[inline]
    fn clip_angle_set(&self, angle: T) -> Self::Output {
        Self::Output {
            phi0: self.phi0,
            phi1: self.phi1,
            base: self.base.clip_angle_set(angle),
        }
    }
}
