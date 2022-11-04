use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::RecenterNoResampling;
use crate::projection::RecenterWithResampling;
use crate::projection::ReflectSet;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoPCNC;
use super::template::ResampleNoneNoPCNC;
use super::template::ResampleNoneNoPCNU;
use super::template::ResampleNonePCNC;
use super::template::ResampleNonePCNU;
use super::template::ResamplePCNC;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPC,
        CLIPU,
        DRAIN,
        NoPCNU,
        PR,
        ResampleNoPCNC<DRAIN, PR, T>,
        ResampleNoPCNU<PR, T>,
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
    fn reflect_x_set(&mut self, reflect: bool) -> &mut Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_with_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: bool) -> &mut Self {
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
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: bool) -> &mut Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_with_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: bool) -> &mut Self {
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
        NoPCNU,
        PR,
        ResampleNoneNoPCNC<DRAIN, PR, T>,
        ResampleNoneNoPCNU<PR, T>,
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
    fn reflect_x_set(&mut self, reflect: bool) -> &mut Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_no_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: bool) -> &mut Self {
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
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: bool) -> &mut Self {
        if reflect {
            self.sx = T::from(-1.0_f64).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter_no_resampling()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: bool) -> &mut Self {
        if reflect {
            self.sy = T::from(-1.0_f64).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter_no_resampling()
    }
}
