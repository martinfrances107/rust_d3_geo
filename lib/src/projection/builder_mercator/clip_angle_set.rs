use core::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleU;
use crate::projection::ClipAngleSet;
use crate::stream::Connectable;
use crate::Transform;

use super::Builder;

impl<DRAIN, PCNC, PCNU, PR, RC, RU, T> ClipAngleSet
    for Builder<ClipAntimeridianU<RC, T>, DRAIN, PCNU, PR, RU, T>
where
    PCNU: Clone + Connectable<Output<DRAIN> = PCNC>,
    RU: Clone + Connectable<Output<PCNC> = RC>,
    PR: Clone + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = Builder<ClipCircleU<RC, T>, DRAIN, PCNU, PR, RU, T>;
    /// f32 or f64.
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle strategy.
    #[inline]
    fn clip_angle_set(&self, angle: T) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            pr: self.pr.clone(),
            base: self.base.clip_angle_set(angle),
            extent: self.extent,
        }
    }
}
