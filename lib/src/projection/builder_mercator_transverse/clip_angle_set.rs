use std::marker::PhantomData;

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
    for Builder<ClipAntimeridianC<RC, T>, ClipAntimeridianU<T>, DRAIN, PCNU, PR, RC, RU, T>
where
    PCNU: Clone + Connectable<Output = PCNC, SC = DRAIN>,
    RC: Clone + Stream<EP = DRAIN, T = T>,
    RU: Clone + Connectable<Output = RC, SC = PCNC> + Debug,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = Builder<ClipCircleC<RC, T>, ClipCircleU<T>, DRAIN, PCNU, PR, RC, RU, T>;
    /// f32 or f64.
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    #[inline]
    fn clip_angle_set(&self, angle: T) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipCircleC<RC, T>>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<RC>,
            pr: self.pr.clone(),
            base: self.base.clip_angle_set(angle),
            extent: self.extent,
        }
    }
}
