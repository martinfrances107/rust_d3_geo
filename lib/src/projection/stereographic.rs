use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::ScaleAdjust;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::template::NoClipC;
use super::builder::template::NoClipU;
use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;
use super::builder::Builder;
use super::ClipAngleSet;
use super::ProjectionRawBase;
use super::ProjectionRawCommon;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct Stereographic<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawCommon<T> for Stereographic<DRAIN, T>
where
    DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
}

impl<DRAIN, T> Default for Stereographic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Stereographic {
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T> ProjectionRawBase<T> for Stereographic<DRAIN, T>
where
    DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type Builder = Builder<
        DRAIN,
        InterpolateCircle<T>,
        LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
        LineCircle<
            DRAIN,
            ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
            Connected<ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>>,
            T,
        >,
        LineCircle<DRAIN, ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>, Unconnected, T>,
        NoClipC<DRAIN, T>,
        NoClipU<DRAIN, T>,
        Stereographic<DRAIN, T>,
        PVCircle<T>,
        ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
        ResampleNoClipU<DRAIN, Stereographic<DRAIN, T>, T>,
        T,
    >;
    #[inline]
    fn builder() -> Self::Builder {
        let clip = gen_clip_antimeridian::<
            DRAIN,
            NoClipC<DRAIN, T>,
            NoClipU<DRAIN, T>,
            Stereographic<DRAIN, T>,
            ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
            ResampleNoClipU<DRAIN, Stereographic<DRAIN, T>, T>,
            T,
        >();

        Builder::new(clip, Stereographic::default())
            .scale(T::from(250_f64).unwrap())
            .clip_angle(T::from(142_f64).unwrap())
    }
}

impl<DRAIN, T> Stereographic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
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
    T: CoordFloat + FloatConst,
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
