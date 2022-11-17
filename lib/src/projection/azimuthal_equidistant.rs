use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::builder::Builder;
use crate::projection::RawBase;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::azimuthal::azimuthal_raw;
use super::ClipAngleSet;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug, Default)]
pub struct AzimuthalEquiDistant<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> RawBase for AzimuthalEquiDistant<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type Builder = BuilderCircleResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(79.4188_f64).unwrap());
        b.clip_angle_set(T::from(180_f64 - 1e-3).unwrap())
    }
}

impl<DRAIN, T> AzimuthalEquiDistant<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn c(c: T) -> T {
        let c = c.acos();
        if c == T::zero() {
            c
        } else {
            c / c.sin()
        }
    }

    #[inline]
    const fn z(z: T) -> T {
        z
    }
}

impl<DRAIN, T> Transform for AzimuthalEquiDistant<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        azimuthal_raw(p, Self::c)
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        azimuthal_invert(p, Self::z)
    }
}
