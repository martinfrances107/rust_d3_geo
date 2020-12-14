use geo::Point;
use num_traits::{float::Float, FloatConst};

use crate::Transform;

#[derive(Clone, Debug)]
pub struct RotationIdentity {}

// By design a stateless function.
// TODO maybe add attributes to suggest inlining this where possible.
fn normalise<T: Float + FloatConst>(p: &Point<T>) -> Point<T> {
    let lambda = p.x();
    let phi = p.y();

    return match lambda.abs() > T::PI() {
        true => Point::new(lambda + (-lambda / T::TAU()).round() * T::TAU(), T::TAU()),
        false => Point::new(lambda, phi),
    };
}

impl RotationIdentity {
    pub fn new<T: Float + FloatConst>() -> Box<dyn Transform<T>> {
        return Box::new(RotationIdentity {});
    }
}

impl<T: Float + FloatConst> Transform<T> for RotationIdentity {
    fn transform(&self, p: &Point<T>) -> Point<T> {
        return normalise(p);
    }

    fn invert(&self, p: &Point<T>) -> Point<T> {
        return normalise(p);
    }
}
