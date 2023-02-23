use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::ScaleSet;
use crate::Transform;

use super::builder::Builder;
use super::BuilderTrait;
use super::RawBase;

/// Equirectangular
/// Used to define a projection builder.
#[derive(Clone, Copy, Debug, Default)]
pub struct Equirectangular<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> RawBase for Equirectangular<DRAIN, T>
where
    DRAIN: Clone + Default,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderAntimeridianResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(152.63_f64).unwrap());
        b
    }
}

impl<DRAIN, T> Transform for Equirectangular<DRAIN, T>
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
