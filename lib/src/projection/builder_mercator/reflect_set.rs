use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::Recenter;
use crate::projection::Reflect;
use crate::projection::ReflectSet;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPU,
        DRAIN,
        Rectangle<Unconnected, T>,
        PR,
        ResamplePCNU<PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self {
        self.base.reflect_x_set(reflect).recenter();
        self
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self {
        self.base.reflect_y_set(reflect).recenter();
        self
    }
}

impl<CLIPU, DRAIN, PR, T> ReflectSet
    for Builder<
        CLIPU,
        DRAIN,
        Rectangle<Unconnected, T>,
        PR,
        ResampleNonePCNU<PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    /// Set the projection builder to invert the x-coordinate.
    #[inline]
    fn reflect_x_set(&mut self, reflect: Reflect) -> &mut Self {
        self.base.reflect_x_set(reflect).recenter();
        self
    }

    /// Set the projection builder to invert the y-coordinate.
    #[inline]
    fn reflect_y_set(&mut self, reflect: Reflect) -> &mut Self {
        self.base.reflect_y_set(reflect).recenter();
        self
    }
}
