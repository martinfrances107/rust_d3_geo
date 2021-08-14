use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

#[derive(Clone, Copy, Default, Debug)]
pub(super) struct RotationLambda<T> {
    pub delta_lambda: T,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda<T: CoordFloat + FloatConst>(
    delta_lambda: T,
    p: &Coordinate<T>,
) -> Coordinate<T> {
    let lambda = p.x + delta_lambda;
    let phi = p.y;
    return match (lambda > T::PI(), lambda < -T::PI()) {
        (false, false) => Coordinate { x: lambda, y: phi }, // -PI <= lambda <= PI
        (true, _) => Coordinate {
            x: lambda - T::TAU(),
            y: phi,
        }, // lambda >  PI
        (_, true) => Coordinate {
            x: lambda + T::TAU(),
            y: phi,
        }, // lambda < -PI
    };
}

impl<'a, T: CoordFloat + FloatConst> RotationLambda<T> {
    #[inline]
    pub fn new(delta_lambda: T) -> RotationLambda<T> {
        Self { delta_lambda }
    }
}

impl<T: CoordFloat + FloatConst> Transform for RotationLambda<T> {
    type C = Coordinate<T>;
    #[inline]
    fn transform(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        forward_rotation_lambda(self.delta_lambda, coordinates)
    }
    #[inline]
    fn invert(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        forward_rotation_lambda(-self.delta_lambda, coordinates)
    }
}
