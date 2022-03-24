use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

// use crate::identity::Identity;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
// use crate::projection::resampler::none::None;
// use crate::clip::clip::Connected as ConnectedClip;
use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;

use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;

use crate::clip::buffer::Buffer;
use crate::clip::clip::Clip;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
// use crate::projection::builder::template::ResampleNoneNoClipC;
// use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::stream::Connected;
use crate::stream::Unconnected;
// use crate::projection::builder::template::DefaultAntimeridian;
use crate::clip::antimeridian::gen_clip_antimeridian;
// use crate::projection::builder::template::Default as DefaultBase;
// use crate::projection::builder::template::DefaultCircle;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::Builder;
use super::ClipAngleSet;
use super::ProjectionRawBase;
use super::ProjectionRawCommon;
use super::Scale;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct Stereographic<DRAIN, T>
// where
//     DRAIN: Stream<EP = DRAIN, T = T> + Default,
//     T: CoordFloat,
{
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawCommon<T> for Stereographic<DRAIN, T>
where
    DRAIN: Stream<EP = DRAIN, T = T> + Default,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
}

impl<DRAIN, T> Default for Stereographic<DRAIN, T>
where
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,
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
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,
    DRAIN: Clone + Debug,
    // T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type Builder = Builder<
        DRAIN,
        InterpolateCircle<DRAIN, ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>, T>,
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
        let clip: Clip<
            DRAIN,
            InterpolateAntimeridian<
                DRAIN,
                ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
                T,
            >,
            LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
            LineAntimeridian<
                DRAIN,
                ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
                Connected<ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>>,
                T,
            >,
            LineAntimeridian<
                DRAIN,
                ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
                Unconnected,
                T,
            >,
            Stereographic<DRAIN, T>,
            PVAntimeridian<T>,
            ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
            ResampleNoClipU<DRAIN, Stereographic<DRAIN, T>, T>,
            Unconnected,
            T,
        > = gen_clip_antimeridian::<
            DRAIN,
            NoClipC<DRAIN, T>,
            NoClipU<DRAIN, T>,
            Stereographic<DRAIN, T>,
            ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
            ResampleNoClipU<DRAIN, Stereographic<DRAIN, T>, T>,
            T,
        >();

        let out_a: Builder<
            DRAIN,
            InterpolateAntimeridian<
                DRAIN,
                ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
                T,
            >,
            LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
            LineAntimeridian<
                DRAIN,
                ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
                Connected<ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>>,
                T,
            >,
            LineAntimeridian<
                DRAIN,
                ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
                Unconnected,
                T,
            >,
            NoClipC<DRAIN, T>,
            NoClipU<DRAIN, T>,
            Stereographic<DRAIN, T>,
            PVAntimeridian<T>,
            ResampleNoClipC<DRAIN, Stereographic<DRAIN, T>, T>,
            ResampleNoClipU<DRAIN, Stereographic<DRAIN, T>, T>,
            T,
        > = Builder::new(clip, Stereographic::default()).scale(T::from(250_f64).unwrap());

        let out_b: Self::Builder = out_a.clip_angle(T::from(142_f64).unwrap());

        out_b
    }
}

impl<DRAIN, T> Stereographic<DRAIN, T>
where
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,

    // T: 'static + CoordFloat + FloatConst,
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
    // DRAIN: Stream<EP = DRAIN, T = T> + Default,
    DRAIN: Clone + Debug,
    // T: 'static + CoordFloat + FloatConst,
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
