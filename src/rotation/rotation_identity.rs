use geo::Coordinate;
use num_traits::{float::Float, FloatConst};

use crate::Transform;

#[derive(Clone, Debug)]
pub struct RotationIdentity {}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise<T: Float + FloatConst>(p: &Coordinate<T>) -> Coordinate<T> {
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
    pub fn new<T: Float + FloatConst>() -> Box<dyn Transform<T>> {
        return Box::new(RotationIdentity {});
    }
}

impl<T: Float + FloatConst> Transform<T> for RotationIdentity {
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        return normalise(p);
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        return normalise(p);
    }
}
