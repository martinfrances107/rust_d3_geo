use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ClipAngleSet;
use crate::stream::Stream;

use super::types::BuilderConicAntimeridianResampleNoClip;
use super::types::BuilderConicCircleResampleNoClip;

impl<DRAIN, PR, T> ClipAngleSet for BuilderConicAntimeridianResampleNoClip<DRAIN, PR, T>
where
    PR: Clone,
    DRAIN: Clone + Stream<EP = DRAIN, T = T>,
    // PR: Clone + PRConic<T = T> + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = BuilderConicCircleResampleNoClip<DRAIN, PR, T>;
    /// f32 or f64.
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    #[inline]
    fn clip_angle_set(&self, angle: T) -> Self::Output {
        Self::Output {
            phi0: self.phi0,
            phi1: self.phi1,
            base: self.base.clip_angle_set(angle),
        }
    }
}
