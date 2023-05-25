use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNU;
use crate::projection::Recenter;
use crate::projection::Reflect;
use crate::projection::ReflectSet;
use crate::stream::Unconnected;
use crate::Transform;

use super::template::ResampleNoneNoPCNU;
use super::template::ResampleNonePCNU;
use super::Builder;

impl<CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPU, DRAIN, Identity<Unconnected>, PR, ResampleNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sx = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sy = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }
}

impl<CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, ResamplePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sx = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sy = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }
}

impl<CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPU, DRAIN, Identity<Unconnected>, PR, ResampleNoneNoPCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sx = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sy = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }
}

impl<CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<CLIPU, DRAIN, Rectangle<Unconnected, T>, PR, ResampleNonePCNU<PR, T>, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sx = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self {
        self.sy = match reflect {
            Reflect::Flipped => T::from(-1.0_f64).unwrap(),
            Reflect::Unflipped => T::one(),
        };
        self.recenter()
    }
}
