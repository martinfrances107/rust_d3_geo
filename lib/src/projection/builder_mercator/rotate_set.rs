use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::RotateSet;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> RotateSet
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

    /// Sets the rotation angles as measured in degrees.
    fn rotate_set(mut self, angles: &[T; 3]) -> Self {
        self.base = self.base.rotate_set(angles);
        self
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> RotateSet
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

    /// Sets the rotation angles as measured in degrees.
    fn rotate_set(mut self, angles: &[T; 3]) -> Self {
        self.base = self.base.rotate_set(angles);
        self
    }
}
