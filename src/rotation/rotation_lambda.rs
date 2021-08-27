use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

/// A single rotation.
#[derive(Clone, Copy, Default, Debug)]
pub struct RotationLambda<T>
where
    T: CoordFloat,
{
    /// The change roations.
    pub delta_lambda: T,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda<T: CoordFloat + FloatConst>(
    delta_lambda: T,
    p: &Coordinate<T>,
) -> Coordinate<T> {
    let lambda = p.x + delta_lambda;
    let phi = p.y;
    match (lambda > T::PI(), lambda < -T::PI()) {
        (false, false) => Coordinate { x: lambda, y: phi }, // -PI <= lambda <= PI
        (true, _) => Coordinate {
            x: lambda - T::TAU(),
            y: phi,
        }, // lambda >  PI
        (_, true) => Coordinate {
            x: lambda + T::TAU(),
            y: phi,
        }, // lambda < -PI
    }
}

impl<'a, T: CoordFloat + FloatConst> RotationLambda<T> {
    /// Contructor.
    #[inline]
    pub fn new(delta_lambda: T) -> RotationLambda<T> {
        Self { delta_lambda }
    }
}

impl<T> Transform for RotationLambda<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        forward_rotation_lambda(self.delta_lambda, coordinate)
    }
    #[inline]
    fn invert(&self, coordinate: &Coordinate<T>) -> Coordinate<T> {
        forward_rotation_lambda(-self.delta_lambda, coordinate)
    }
}
