use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::circle::interpolate::generate as gen_interpolate;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::Builder;
use super::Raw;
use super::Scale;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct Stereographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat,
{
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> Default for Stereographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Stereographic {
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T> Raw<T> for Stereographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, Line<T>, Stereographic<DRAIN, T>, PV<T>, T>;
    type T = T;

    #[inline]
    fn builder() -> Self::Builder
    where
        DRAIN: Stream<T = T>,
    {
        Builder::new(
            StreamNodeClipFactory::new(
                gen_interpolate(T::one()),
                Line::<T>::default(),
                PV::default(),
            ),
            Stereographic::default(),
        )
        .scale(T::from(250_f64).unwrap())
        .clip_angle(T::from(142_f64).unwrap())
    }
}

impl<DRAIN, T> Stereographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    #[inline]
    fn z(z: T) -> T
    where
        T: CoordFloat + FloatConst,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        T::from(2).unwrap() * z.atan()
    }
}

impl<DRAIN, T> Transform for Stereographic<DRAIN, T>
where
    DRAIN: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = T::one() + p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, Self::z)
    }
}
