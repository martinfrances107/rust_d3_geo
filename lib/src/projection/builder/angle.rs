use crate::projection::RecenterWithResampling;
use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::AngleSet;
use crate::Transform;

use super::template::ClipU;
use super::template::NoClipU;
use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> AngleSet
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
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn angle_set(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> AngleSet
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
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    fn angle_set(mut self, angle: T) -> Self {
        self.alpha = (angle % T::from(360_f64).unwrap()).to_radians();
        self.recenter_with_resampling()
    }
}
