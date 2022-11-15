use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::Transform;

use super::builder_identity::types::BuilderIdentityAntimeridianResampleNoClip;
use super::builder_identity::Builder;
use super::RawBase;

/// Projection definition.
#[derive(Clone, Copy, Default, Debug)]
pub struct Identity<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> RawBase for Identity<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderIdentityAntimeridianResampleNoClip<DRAIN, T>;

    #[inline]
    fn builder() -> Self::Builder {
        Builder::default()
    }
}

impl<DRAIN, T> Transform for Identity<DRAIN, T>
where
    T: CoordFloat,
{
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
