use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::ClipAngleSet;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::Transform;

use super::Builder;

impl<DRAIN, PCNC, PCNU, PR, RC, RU, T> ClipAngleSet
    for Builder<ClipAntimeridianC<RC, T>, ClipAntimeridianU<RC, T>, PCNU, PR, RU, T>
where
    DRAIN: Clone,
    PCNC: Clone,
    PCNU: Clone + Connectable<Output<DRAIN> = PCNC>,
    RC: Clone + Stream<EP = DRAIN, T = T>,
    RU: Clone + Connectable<Output<PCNC> = RC> + Debug,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = Builder<ClipCircleC<RC, T>, ClipCircleU<RC, T>, PCNU, PR, RU, T>;
    /// f32 or f64.
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    #[inline]
    fn clip_angle_set(&self, angle: T) -> Self::Output {
        Self::Output {
            pr: self.pr.clone(),
            base: self.base.clip_angle_set(angle),
            extent: self.extent,
        }
    }
}
