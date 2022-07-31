use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::math::asin;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::builder::Builder;
use crate::projection::ClipAngleSet;
use crate::projection::ProjectionRawBase;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::azimuthal::azimuthal_raw;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug, Default)]
pub struct AzimuthalEqualArea<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> ProjectionRawBase for AzimuthalEqualArea<DRAIN, T>
where
    DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    type Builder = BuilderCircleResampleNoClip<DRAIN, AzimuthalEqualArea<DRAIN, T>, T>;

    #[inline]
    fn builder() -> Self::Builder {
        Builder::new(AzimuthalEqualArea::default())
            .scale_set(T::from(124.75_f64).unwrap())
            .clip_angle_set(T::from(180_f64 - 1e-3).unwrap())
    }
}

impl<DRAIN, T> AzimuthalEqualArea<DRAIN, T>
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
        two * asin(z / two)
    }
}

impl<DRAIN, T> Transform for AzimuthalEqualArea<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_raw(p, Self::cxcy)
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, Self::z)
    }
}
