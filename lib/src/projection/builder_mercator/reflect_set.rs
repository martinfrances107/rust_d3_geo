use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::RecenterNoResampling;
use crate::projection::RecenterWithResampling;
use crate::projection::ReflectSet;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
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

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_x_set(reflect).recenter_with_resampling(),
            ..self
        }
    }

    /// Set the projection builder to invert the y-coordinate.    
    #[inline]
    fn reflect_y_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_y_set(reflect).recenter_with_resampling(),
            ..self
        }
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
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

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_x_set(reflect).recenter_with_resampling(),
            ..self
        }
    }

    /// Set the projection builder to invert the y-coordinate.    
    #[inline]
    fn reflect_y_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_y_set(reflect).recenter_with_resampling(),
            ..self
        }
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
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
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_x_set(reflect).recenter_no_resampling(),
            ..self
        }
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_y_set(reflect).recenter_no_resampling(),
            ..self
        }
    }
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
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
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_x_set(reflect).recenter_no_resampling(),
            ..self
        }
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(self, reflect: bool) -> Self {
        Self {
            base: self.base.reflect_y_set(reflect).recenter_no_resampling(),
            ..self
        }
    }
}
