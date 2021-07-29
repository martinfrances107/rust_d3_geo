use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use super::azimuthal::azimuthal_invert;
use super::projection::Projection;
// use super::projection::StreamOrValueMaybe;
use crate::projection::projection_trait::ProjectionTrait;
use crate::projection::scale::Scale;
use crate::stream::Stream;
use crate::Transform;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct Stereographic<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Default for Stereographic<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Stereographic {
            phantom: PhantomData::<T>,
        }
    }
}

impl<T> Stereographic<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    pub fn gen_projection_mutator<'a, DRAIN>() -> Projection<'a, DRAIN, Stereographic<T>, T>
    where
        DRAIN: 'a + Default + Stream<SC = Coordinate<T>>,
    {
        Projection::new(Stereographic::default(), None)
            .scale(T::from(250f64).unwrap())
            // .clip_angle(StreamOrValueMaybe::Value(T::from(142f64).unwrap()))
            .clip_angle(T::from(142f64).unwrap())
    }

    #[inline]
    fn z(z: T) -> T
    where
        T: CoordFloat + FloatConst,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        T::from(2).unwrap() * z.atan()
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Transform
    for Stereographic<T>
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = T::one() + p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, Self::z)
    }
}
