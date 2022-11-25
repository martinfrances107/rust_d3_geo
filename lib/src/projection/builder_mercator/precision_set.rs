use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
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
        ClipAntimeridianC<None<PR, Connected<PCNC>, T>, T>,
        ClipAntimeridianU<None<PR, Connected<PCNC>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        None<PR, Connected<PCNC>, T>,
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
        ClipAntimeridianC<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        ClipAntimeridianU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, ConnectedResample<PCNC, T>, T>,
        Resample<PR, Unconnected, T>,
        T,
    >;
    type T = T;

    #[inline]
    fn precision_set(&self, delta: &T) -> Self::Output {
        Self::Output {
            extent: self.extent,
            pr: self.pr.clone(),
            base: self.base.precision_set(delta),
        }
    }
}

impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
    for Builder<
        ClipCircleC<None<PR, Connected<PCNC>, T>, T>,
        ClipCircleU<None<PR, Connected<PCNC>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        None<PR, Connected<PCNC>, T>,
        None<PR, Unconnected, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    PCNC: Clone,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Output = Builder<
        ClipCircleC<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        ClipCircleU<Resample<PR, ConnectedResample<PCNC, T>, T>, T>,
        DRAIN,
        PCNU,
        PR,
        Resample<PR, ConnectedResample<PCNC, T>, T>,
        Resample<PR, Unconnected, T>,
        T,
    >;
    type T = T;

    #[inline]
    fn precision_set(&self, _delta: &T) -> Self::Output {
        todo!();
        // Self::Output {
        //     p_clipc: PhantomData::<ClipCircleC<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>, T>>,
        //     p_drain: PhantomData::<DRAIN>,
        //     p_rc: PhantomData::<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>>,
        //     extent: self.extent,
        //     pr: self.pr.clone(),
        //     base: self.base.precision_set(delta),
        // }
    }
}
