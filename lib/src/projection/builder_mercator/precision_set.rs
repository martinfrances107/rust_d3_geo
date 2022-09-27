// use std::marker::PhantomData;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use num_traits::FloatConst;

// use crate::clip::antimeridian::ClipAntimeridianC;
// use crate::clip::antimeridian::ClipAntimeridianU;
// use crate::clip::circle::ClipCircleC;
// use crate::clip::circle::ClipCircleU;
// use crate::projection::resampler::none::None;
// use crate::projection::resampler::resample::Connected as ConnectedResample;
// use crate::projection::resampler::resample::Resample;
// use crate::projection::PrecisionSet;
// use crate::stream::Connected;
// use crate::stream::Unconnected;
// use crate::Transform;

// use super::Builder;

// impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
//     for Builder<
//         ClipAntimeridianC<None<PR, PCNC, Connected<PCNC>, T>, T>,
//         ClipAntimeridianU<None<PR, PCNC, Connected<PCNC>, T>, T>,
//         DRAIN,
//         PCNU,
//         PR,
//         None<PR, PCNC, Connected<PCNC>, T>,
//         None<PR, PCNC, Unconnected, T>,
//         T,
//     >
// where
//     PCNC: Clone,
//     PR: Clone + Transform<T = T>,
//     T: CoordFloat + Default + FloatConst,
// {
//     type Output = Builder<
//         ClipAntimeridianC<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>, T>,
//         ClipAntimeridianU<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>, T>,
//         DRAIN,
//         PCNU,
//         PR,
//         Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>,
//         Resample<PR, PCNC, Unconnected, T>,
//         T,
//     >;
//     type T = T;

//     #[inline]
//     fn precision_set(self, delta: &T) -> Self::Output {
//         Self::Output {
//             p_clipc: PhantomData::<
//                 ClipAntimeridianC<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>, T>,
//             >,
//             p_drain: PhantomData::<DRAIN>,
//             p_rc: PhantomData::<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>>,
//             extent: self.extent,
//             pr: self.pr,
//             base: self.base.precision_set(delta),
//         }
//     }
// }

// impl<DRAIN, PR, PCNC, PCNU, T> PrecisionSet
//     for Builder<
//         ClipCircleC<None<PR, PCNC, Connected<PCNC>, T>, T>,
//         ClipCircleU<None<PR, PCNC, Connected<PCNC>, T>, T>,
//         DRAIN,
//         PCNU,
//         PR,
//         None<PR, PCNC, Connected<PCNC>, T>,
//         None<PR, PCNC, Unconnected, T>,
//         T,
//     >
// where
//     PR: Clone + Transform<T = T>,
//     PCNC: Clone,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type Output = Builder<
//         ClipCircleC<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>, T>,
//         ClipCircleU<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>, T>,
//         DRAIN,
//         PCNU,
//         PR,
//         Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>,
//         Resample<PR, PCNC, Unconnected, T>,
//         T,
//     >;
//     type T = T;

//     #[inline]
//     fn precision_set(self, delta: &T) -> Self::Output {
//         Self::Output {
//             p_clipc: PhantomData::<ClipCircleC<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>, T>>,
//             p_drain: PhantomData::<DRAIN>,
//             p_rc: PhantomData::<Resample<PR, PCNC, ConnectedResample<PCNC, T>, T>>,
//             extent: self.extent,
//             pr: self.pr,
//             base: self.base.precision_set(delta),
//         }
//     }
// }
