use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

#[derive(Clone, Debug)]
pub struct RotationIdentity {}

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

impl RotationIdentity {
    #[inline]
    pub fn new<T: CoordFloat + FloatConst>() -> Box<dyn Transform<T>> {
        Box::new(RotationIdentity {})
    }
}

impl<T: CoordFloat + FloatConst> Transform<T> for RotationIdentity {
    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        normalise(p)
    }
}
