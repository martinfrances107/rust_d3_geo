use geo::CoordFloat;

use crate::projection::builder::Builder;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleClipC;
use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::RecenterWithResampling;
use crate::projection::ScaleSet;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ScaleSet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        NoClipU<DRAIN>,
        PR,
        PV,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
        T,
    >
where
    Self: RecenterWithResampling,
    T: CoordFloat,
{
    type T = T;

    fn scale_set(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ScaleSet
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Clone,
    Self: RecenterWithResampling,
    T: CoordFloat,
{
    type T = T;

    fn scale_set(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_with_resampling()
    }
}
