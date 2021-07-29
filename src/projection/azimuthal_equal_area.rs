use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use super::projection::Projection;
// use super::projection::StreamOrValueMaybe;
use super::azimuthal::azimuthal_invert;
use super::azimuthal::azimuthal_raw;
use crate::projection::projection_trait::ProjectionTrait;
use crate::projection::scale::Scale;
use crate::stream::Stream;
use crate::Transform;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct AzimuthalEqualArea<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Default for AzimuthalEqualArea<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        AzimuthalEqualArea {
            phantom: PhantomData::<T>,
        }
    }
}
impl<T> AzimuthalEqualArea<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    pub fn gen_projection_mutator<'a, DRAIN>() -> Projection<'a, DRAIN, AzimuthalEqualArea<T>, T>
    where
        DRAIN: 'a + Default + Stream<SC = Coordinate<T>>,
    {
        Projection::new(AzimuthalEqualArea::default(), None)
            .scale(T::from(124.75f64).unwrap())
            // .clip_angle(StreamOrValueMaybe::Value(T::from(180f64 - 1e-3).unwrap()))
            .clip_angle(T::from(180f64 - 1e-3).unwrap())
    }

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

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Transform
    for AzimuthalEqualArea<T>
{
    type C = Coordinate<T>;
    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_raw(p, Self::cxcy)
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, Self::z)
    }
}
