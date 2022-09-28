use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::ResampleNoPCNC;
use crate::projection::builder::template::ResampleNoPCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::RotateSet;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU<DRAIN>,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<DRAIN, PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
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

impl<CLIPC, CLIPU, DRAIN, PR, T> RotateSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<DRAIN, T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<DRAIN, PR, T>,
        T,
    >
where
    CLIPC: Clone,
    CLIPU: Clone,
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
