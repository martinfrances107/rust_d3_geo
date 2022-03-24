use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

// use crate::clip::buffer::Buffer;
// use crate::clip::Bufferable;
// use crate::clip::Interpolator;
// use crate::clip::LineConnected;
// use crate::clip::LineUnconnected;
// use crate::clip::PointVisible;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
// use crate::projection::resampler::Resampler;
use crate::projection::Center;
// use crate::projection::ProjectionRawBase;
// use crate::stream::Connectable;
use crate::stream::Connected;
use crate::Transform;
// use crate::stream::Stream;
use crate::stream::Unconnected;

use super::Builder;
// use super::PostClipNode;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> Center
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    DRAIN: Clone + Debug,
    I: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PV: Clone + Debug,
    PCNC: Clone + Debug,
    PCNU: Clone + Debug,
    // I: Interpolator<
    //     EP = DRAIN,
    //     Stream = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
    //     T = T,
    // >,
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>
    //     + Stream<EP = DRAIN, T = T>,
    // LU: LineUnconnected<SU = Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>>
    //     + Connectable<
    //         Output = LC,
    //         SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
    //     > + Bufferable<Output = LB, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PR: ProjectionRawBase<T>,
    PR: Transform<T = T>,
    // PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        }
    }

    fn center(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, INTERPOLATE, LB, LC, LU, PCNC, PCNU, PR, PV, T> Center
    for Builder<
        DRAIN,
        INTERPOLATE,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    // INTERPOLATE: Interpolator<
    //     EP = DRAIN,
    //     Stream = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
    //     T = T,
    // >,
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
    //     + Stream<EP = DRAIN, T = T>,
    // LU: LineUnconnected<SU = ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>>
    //     + Connectable<Output = LC, SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
    //     + Bufferable<Output = LB, T = T>,
    // PCNC: PostClipNode + Stream<EP = DRAIN, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = DRAIN>,
    // PR: ProjectionRawBase<T>,
    INTERPOLATE: Clone,
    DRAIN: Clone + Debug,
    PV: Clone,
    PCNU: Clone + Debug,
    LB: Clone,
    LC: Clone + Debug,
    LU: Clone + Debug,
    PV: Clone + Debug,
    PR: Clone + Transform<T = T>,
    // PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn get_center(&self) -> Coordinate<T> {
        Coordinate {
            x: self.lambda.to_degrees(),
            y: self.phi.to_degrees(),
        }
    }

    fn center(mut self, p: &Coordinate<T>) -> Self {
        self.lambda = (p.x % T::from(360_u16).unwrap()).to_radians();
        self.phi = (p.y % T::from(360_u16).unwrap()).to_radians();
        self.recenter_no_resampling()
    }
}
