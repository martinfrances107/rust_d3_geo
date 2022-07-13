use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ClipAngleSet;
use crate::projection::Scale;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::template::NoClipU;
use super::builder::Builder;
use super::ProjectionRawBase;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct Stereographic<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
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

impl<DRAIN, T> ProjectionRawBase for Stereographic<DRAIN, T>
where
    DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Builder = BuilderCircleResampleNoClip<DRAIN, Stereographic<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let clip = gen_clip_antimeridian::<NoClipU<DRAIN>, _, _>();

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
