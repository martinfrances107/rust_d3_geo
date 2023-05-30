use core::fmt::Debug;
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
#[derive(Clone, Debug, Default)]
pub struct Equirectangular<T> {
    p_t: PhantomData<T>,
}

impl<T> RawBase for Equirectangular<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = BuilderAntimeridianResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(152.63_f64).unwrap());
        b
    }
}

impl<T> Transform for Equirectangular<T>
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
