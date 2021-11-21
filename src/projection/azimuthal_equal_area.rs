use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::gen_clip_factory_antimeridian;
use crate::clip::circle::pv::PV;
use crate::projection::builder::Builder;
use crate::projection::Raw;
use crate::projection::Scale;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::azimuthal::azimuthal_raw;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct AzimuthalEqualArea<DRAIN, EP, T>
where
    DRAIN: Stream<EP = EP, T = T>,
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, EP, T> Raw<T> for AzimuthalEqualArea<DRAIN, EP, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type Builder = Builder<DRAIN, EP, AzimuthalEqualArea<DRAIN, EP, T>, PV<T>, T>;
    type T = T;

    #[inline]
    fn builder() -> Self::Builder {
        Builder::new(
            gen_clip_factory_antimeridian(),
            AzimuthalEqualArea::default(),
        )
        .scale(T::from(124.75_f64).unwrap())
        .clip_angle(T::from(180_f64 - 1e-3).unwrap())
    }
}

impl<DRAIN, EP, T> Default for AzimuthalEqualArea<DRAIN, EP, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        AzimuthalEqualArea {
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, EP, T> AzimuthalEqualArea<DRAIN, EP, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn cxcy(cxcy: T) -> T {
        (T::from(2).unwrap() / (T::one() + cxcy)).sqrt()
    }

    #[inline]
    fn z(z: T) -> T {
        let two = T::from(2.0).unwrap();
        two * (z / two).asin()
    }
}

impl<DRAIN, EP, T> Transform for AzimuthalEqualArea<DRAIN, EP, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    DRAIN: Stream<EP = EP, T = T>,
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
