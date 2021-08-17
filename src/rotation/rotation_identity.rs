use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
///
/// Raw is generic over T ( Raw<T=T> )
///
#[derive(Clone, Copy, Debug)]
pub struct RotationIdentity<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise<T: CoordFloat + FloatConst>(p: &Coordinate<T>) -> Coordinate<T> {
    let lambda = p.x;
    let phi = p.y;

    let x = match lambda.abs() > T::PI() {
        // true => Coordinate {
        //     x: lambda + (-lambda / T::TAU()).round() * T::TAU(),
        //     y: T::TAU(),
        // },
        // false => Coordinate { x: lambda, y: phi },
        true => lambda + (-lambda / T::TAU()).round() * T::TAU(),
        false => lambda,
    };
    Coordinate { x, y: phi }
}

impl<T> Default for RotationIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData::<T>,
        }
    }
}

impl<T> Transform for RotationIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }
}
