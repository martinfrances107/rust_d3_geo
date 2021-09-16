use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::circle::interpolate::generate as gen_interpolate;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::Raw;
use super::Scale;

/// Orthographic
///
/// Root transform.
/// Used to define a projection builder.
#[derive(Clone, Copy, Debug)]
pub struct Orthographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Orthographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Orthographic {
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T> Raw<T> for Orthographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, Line<T>, Orthographic<DRAIN, T>, PV<T>, T>;
    type T = T;
    #[inline]
    fn builder() -> Builder<DRAIN, Line<T>, Orthographic<DRAIN, T>, PV<T>, T>
    where
        DRAIN: Stream<T = T>,
    {
        Builder::new(
            StreamNodeClipFactory::new(
                gen_interpolate(T::one()),
                Line::<T>::default(),
                PV::default(),
            ),
            Orthographic::default(),
        )
        .scale(T::from(249.5_f64).unwrap())
        .clip_angle(T::from(90_f64 + 1e-6_f64).unwrap())
    }
}

impl<DRAIN, T> Orthographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
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
    DRAIN: Stream<T = T>,
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
