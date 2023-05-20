use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

use super::builder_identity::types::BuilderIdentityAntimeridianResampleNoClip;
use super::builder_identity::Builder;
use super::RawBase;

/// Projection definition.
#[derive(Clone, Default, Debug)]
pub struct Identity<T> {
    p_t: PhantomData<T>,
}

impl<T> RawBase for Identity<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = BuilderIdentityAntimeridianResampleNoClip<T>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        Builder::default()
    }
}

impl<T> Transform for Identity<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        *p
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        *p
    }
}
