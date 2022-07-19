use geo::CoordFloat;

use crate::projection::builder::Builder;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleNoneClipC;
use crate::projection::builder::ResampleNoneClipU;
use crate::projection::builder::ResampleNoneNoClipC;
use crate::projection::builder::ResampleNoneNoClipU;
use crate::projection::RecenterNoResampling;
use crate::projection::Scale;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> Scale
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        NoClipU<DRAIN>,
        PR,
        PV,
        ResampleNoneNoClipC<DRAIN, PR, T>,
        ResampleNoneNoClipU<DRAIN, PR, T>,
        T,
    >
where
    Self: RecenterNoResampling,
    T: CoordFloat,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> Scale
    for Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        ClipU<DRAIN, T>,
        PR,
        PV,
        ResampleNoneClipC<DRAIN, PR, T>,
        ResampleNoneClipU<DRAIN, PR, T>,
        T,
    >
where
    DRAIN: Clone,
    Self: RecenterNoResampling,
    T: CoordFloat,
{
    type T = T;

    fn scale(mut self, scale: T) -> Self {
        self.k = scale;
        self.recenter_no_resampling()
    }
}
