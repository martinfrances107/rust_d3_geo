use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::ScaleSet;
use crate::Transform;

use super::builder::Builder;
use super::ProjectionRawBase;

/// Equirectangular
/// Used to define a projection builder.
#[derive(Clone, Copy, Debug, Default)]
pub struct Equirectangular<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawBase for Equirectangular<DRAIN, T>
where
    DRAIN: Clone + Default,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderAntimeridianResampleNoClip<DRAIN, Equirectangular<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        Builder::new(Equirectangular::default()).scale_set(T::from(152.63_f64).unwrap())
    }
}

impl<DRAIN, T> Transform for Equirectangular<DRAIN, T>
where
    T: CoordFloat,
{
    /// f64 or f32.
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        *p
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        *p
    }
}
