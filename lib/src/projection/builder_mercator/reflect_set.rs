use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::RecenterNoResampling;
use crate::projection::RecenterWithResampling;
use crate::projection::ReflectSet;
use crate::projection::REFLECT;
use crate::Transform;

use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResamplePCNC<DRAIN, PR, T>,
        ResamplePCNU<PR, T>,
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

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.base.reflect_x_set(reflect).recenter_with_resampling();
        self
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.base.reflect_y_set(reflect).recenter_with_resampling();
        self
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        PCNU<T>,
        PR,
        ResampleNonePCNC<DRAIN, PR, T>,
        ResampleNonePCNU<PR, T>,
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

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.base.reflect_x_set(reflect).recenter_no_resampling();
        self
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.base.reflect_y_set(reflect).recenter_no_resampling();
        self
    }
}
