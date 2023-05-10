use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::Recenter;
use crate::projection::ReflectSet;
use crate::projection::REFLECT;
use crate::Transform;

use super::template::NoPCNU;
use super::template::ResampleNoneNoPCNU;
use super::template::ResampleNonePCNU;
use super::template::PCNU;
use super::Builder;

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPC, CLIPU, DRAIN, NoPCNU, PR, ResampleNoPCNU<PR, T>, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sx = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sy = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sx = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sy = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPC, CLIPU, DRAIN, NoPCNU, PR, ResampleNoneNoPCNU<PR, T>, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sx = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sy = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }
}

impl<CLIPC, CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPC, CLIPU, DRAIN, PCNU<T>, PR, ResampleNonePCNU<PR, T>, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sx = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: REFLECT) -> &mut Self {
        self.sy = match reflect {
            REFLECT::Flipped => T::from(-1.0_f64).unwrap(),
            REFLECT::Unflipped => T::one(),
        };
        self.recenter()
    }
}
