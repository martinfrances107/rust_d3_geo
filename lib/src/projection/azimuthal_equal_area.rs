use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::builder::Builder;
use crate::projection::RawBase;
use crate::projection::ScaleSet;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::azimuthal::azimuthal_raw;
use super::BuilderTrait;
use super::ClipAngleSet;

/// Projection definition.
#[derive(Copy, Clone, Debug, Default)]
pub struct AzimuthalEqualArea<T>
where
    T: CoordFloat,
{
    // Why the Phantom Data is required here...
    //
    // The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
    // The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    p_t: PhantomData<T>,
}

impl<T> RawBase for AzimuthalEqualArea<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = BuilderCircleResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(124.75_f64).unwrap());
        b.clip_angle_set(T::from(180_f64 - 1e-3).unwrap())
    }
}

impl<T> AzimuthalEqualArea<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn cxcy(cxcy: T) -> T {
        (T::from(2).unwrap() / (T::one() + cxcy)).sqrt()
    }

    #[inline]
    fn z(z: T) -> T {
        let two = T::from(2.0_f64).unwrap();
        two * (z / two).asin()
    }
}

impl<T> Transform for AzimuthalEqualArea<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        azimuthal_raw(p, Self::cxcy)
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        azimuthal_invert(p, Self::z)
    }
}
