use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::PrecisionSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
    for Builder<
        ClipAntimeridianU<None<PR, Connected<PCNC>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        None<PR, Unconnected, T>,
        T,
    >
where
    PCNC: Clone,
    PCNU: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Output = Builder<
        ClipAntimeridianU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, Unconnected, T>,
        T,
    >;
    type T = T;

    #[inline]
    fn precision_set(&self, delta: &T) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            extent: self.extent,
            pr: self.pr.clone(),
            base: self.base.precision_set(delta),
        }
    }
}

impl<DRAIN, PR, PCNU, T> PrecisionSet
    for Builder<
        ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
        DRAIN,
        PCNU,
        PR,
        None<PR, Unconnected, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    PCNU: Clone,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = Builder<
        ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, Unconnected, T>,
        T,
    >;
    type T = T;

    #[inline]
    fn precision_set(&self, delta: &T) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            extent: self.extent,
            pr: self.pr.clone(),
            base: self.base.precision_set(delta),
        }
    }
}
