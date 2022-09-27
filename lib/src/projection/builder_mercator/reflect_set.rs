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

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoClipU<DRAIN>,
        PR,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
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

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        ClipU<DRAIN, T>,
        PR,
        ResampleClipC<DRAIN, PR, T>,
        ResampleClipU<DRAIN, PR, T>,
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

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoClipU<DRAIN>,
        PR,
        ResampleNoneNoClipC<DRAIN, PR, T>,
        ResampleNoneNoClipU<DRAIN, PR, T>,
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

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        ClipU<DRAIN, T>,
        PR,
        ResampleNoneClipC<DRAIN, PR, T>,
        ResampleNoneClipU<DRAIN, PR, T>,
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
