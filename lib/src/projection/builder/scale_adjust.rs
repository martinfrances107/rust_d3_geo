use crate::projection::builder::NoClipC;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use std::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::Bufferable;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::Buffer;
use crate::projection::builder::ClipC;
use crate::projection::builder::ClipU;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ScaleAdjust;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LC, LB, LU, PCNC, PCNU, PR, PV, T> ScaleAdjust
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        NoClipC<DRAIN, T>,
        NoClipU<DRAIN, T>,
        PR,
        PV,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Debug,
    I: Interpolator<T = T>,
    LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone
        + LineConnected<SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>
        + Stream<EP = DRAIN, T = T>,
    LU: Clone
        + Connectable<
            Output = LC,
            SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        > + Bufferable<Output = LB, T = T>
        + Debug,
    PCNC: Debug,
    PCNU: Debug,
    PR: Clone + Debug + Transform<T = T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LC, LB, LU, PCNC, PCNU, PR, PV, T> ScaleAdjust
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipC<DRAIN, T>,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Debug,
    I: Interpolator<T = T>,
    LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone
        + LineConnected<SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>>
        + Stream<EP = DRAIN, T = T>,
    LU: Clone
        + Connectable<
            Output = LC,
            SC = Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        > + Bufferable<Output = LB, T = T>
        + Debug,
    PCNC: Debug,
    PCNU: Debug,
    PR: Clone + Debug + Transform<T = T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T> ScaleAdjust
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
        ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    DRAIN: Debug,
    I: Interpolator<T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: Clone
        + LineConnected<SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
        + Stream<EP = DRAIN, T = T>,
    LU: Clone
        + Connectable<Output = LC, SC = ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>>
        + Bufferable<Output = LB, T = T>
        + Debug,
    PR: Clone + Debug + Transform<T = T>,
    PCNC: Debug,
    PCNU: Debug,
    PV: PointVisible<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}
