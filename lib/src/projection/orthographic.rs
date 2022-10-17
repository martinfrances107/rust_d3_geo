use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::math::EPSILON;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::ClipAngleSet;
use super::ProjectionRawBase;

/// Projection definition.
#[derive(Clone, Copy, Default, Debug)]
pub struct Orthographic<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawBase for Orthographic<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    type Builder = BuilderCircleResampleNoClip<DRAIN, Orthographic<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let mut b = Builder::new(Orthographic::default());
        b.scale_set(T::from(249.5_f64).unwrap());
        b.clip_angle_set(T::from(90_f64 + EPSILON).unwrap())
    }
}

impl<DRAIN, T> Orthographic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn angle(z: T) -> T {
        z.asin()
    }

    fn azimuthal_invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let z = (p.x * p.x + p.y * p.y).sqrt();
        let c = Orthographic::<DRAIN, T>::angle(z);
        let sc = c.sin();
        let cc = c.cos();

        let ret_x = (p.x * sc).atan2(z * cc);

        let y_out = if z == T::zero() { z } else { p.y * sc / z };
        let ret_y = y_out.asin();

        Coordinate { x: ret_x, y: ret_y }
    }
}

impl<DRAIN, T> Transform for Orthographic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: p.y.cos() * p.x.sin(),
            y: p.y.sin(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        self.azimuthal_invert(p)
    }
}
