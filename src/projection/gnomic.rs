use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::projection::Projection;
use super::projection_trait::ProjectionTrait;
use super::scale::Scale;
// use crate::stream::Stream;
// use super::projection::StreamOrValueMaybe;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct GnomicRaw<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Default for GnomicRaw<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData::<T>,
            // lambda: T::zero(),
            // phi: T::zero(),
        }
    }
}

impl<T> GnomicRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn gen_projection_mutator<'a>() -> Projection<'a, GnomicRaw<T>, T>
// where
        // SD: 'a + Stream<SC = Coordinate<T>> + Default,
    {
        let g = GnomicRaw::default();
        let projection = Projection::new(g, None);
        projection
            .scale(T::from(144.049f64).unwrap())
            // .clip_angle(StreamOrValueMaybe::Value(T::from(60f64).unwrap()))
            .clip_angle(T::from(60f64).unwrap())
    }

    #[inline]
    fn atan(z: T) -> T
    where
        T: CoordFloat + FloatConst,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        z.atan()
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Transform for GnomicRaw<T> {
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, Self::atan)
    }
}
