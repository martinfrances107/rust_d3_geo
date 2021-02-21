use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::marker::PhantomData;

use crate::Transform;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Debug, Default)]
pub struct RotationIdentity<T>
where
    T: CoordFloat + std::default::Default,
{
    phantom: PhantomData<T>,
}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise<T: CoordFloat + FloatConst>(p: &Coordinate<T>) -> Coordinate<T> {
    let lambda = p.x;
    let phi = p.y;

    return match lambda.abs() > T::PI() {
        true => Coordinate {
            x: lambda + (-lambda / T::TAU()).round() * T::TAU(),
            y: T::TAU(),
        },
        false => Coordinate { x: lambda, y: phi },
    };
}

impl<T> RotationIdentity<T>
where
    T: CoordFloat + FloatConst + std::default::Default + 'static,
{
    #[inline]
    pub fn new() -> Box<dyn Transform<C = Coordinate<T>>> {
        Box::new(RotationIdentity::<T>::default())
    }
}

impl<T: CoordFloat + FloatConst + std::default::Default> Transform for RotationIdentity<T> {
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
