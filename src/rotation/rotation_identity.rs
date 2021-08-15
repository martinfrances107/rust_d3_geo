use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug)]
pub struct RotationIdentity<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise<'a, T: CoordFloat + FloatConst>(p: &'a Coordinate<T>) -> Coordinate<T> {
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

impl<T: CoordFloat> Default for RotationIdentity<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::<T>,
        }
    }
}

impl<T: CoordFloat + FloatConst> Transform for RotationIdentity<T> {
    type C = Coordinate<T>;
    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }
}
