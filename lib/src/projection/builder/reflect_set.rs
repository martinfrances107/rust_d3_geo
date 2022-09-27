use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::RecenterNoResampling;
use crate::projection::RecenterWithResampling;
use crate::projection::ReflectSet;
use crate::Transform;

use super::template::ClipU;
use super::template::NoClipU;
use super::template::ResampleClipC;
use super::template::ResampleNoClipC;
use super::template::ResampleNoneClipC;
use super::template::ResampleNoneClipU;
use super::template::ResampleNoneNoClipC;
use super::template::ResampleNoneNoClipU;
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
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_with_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sy = T::from(-1.0_f64).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter_with_resampling()
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
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_with_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sy = T::from(-1.0_f64).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter_with_resampling()
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
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_no_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sy = T::from(-1.0_f64).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter_no_resampling()
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
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_no_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(mut self, reflect: bool) -> Self {
        if reflect {
            self.sy = T::from(-1.0_f64).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter_no_resampling()
    }
}
