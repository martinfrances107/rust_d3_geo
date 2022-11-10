use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::Transform;

/// A single rotation.
#[derive(Clone, Copy, Default, Debug)]
pub struct RotationLambda<T> {
    /// The change in rotation.
    pub delta_lambda: T,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda<T: CoordFloat + FloatConst>(
    delta_lambda: T,
    p: &Coordinate<T>,
) -> Coordinate<T> {
    let mut lambda = p.x + delta_lambda;
    if lambda.abs() > T::PI() {
        lambda = lambda - (lambda / T::TAU()).round() * T::TAU();
    }
    Coordinate { x: lambda, y: p.y }
}

impl<T: CoordFloat + FloatConst> RotationLambda<T> {
    /// Constructor.
    #[inline]
    pub const fn new(delta_lambda: T) -> Self {
        Self { delta_lambda }
    }
}

impl<T> Transform for RotationLambda<T>
where
    T: CoordFloat + FloatConst,
{
    /// f64 or f32.
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
