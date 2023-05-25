use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::Transform;

/// A single rotation.
#[derive(Clone, Default, Debug)]
pub struct RotationLambda<T> {
    /// The change in rotation.
    delta_lambda: T,
}

fn forward_rotation_lambda<T: CoordFloat + FloatConst>(delta_lambda: T, p: &Coord<T>) -> Coord<T> {
    let mut lambda = p.x + delta_lambda;
    if lambda.abs() > T::PI() {
        lambda = lambda - (lambda / T::TAU()).round() * T::TAU();
    }
    Coord { x: lambda, y: p.y }
}

impl<T: CoordFloat + FloatConst> RotationLambda<T> {
    /// Constructor.
    #[inline]
    pub(super) const fn new(delta_lambda: T) -> Self {
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
    fn transform(&self, coordinate: &Coord<T>) -> Coord<T> {
        forward_rotation_lambda(self.delta_lambda, coordinate)
    }
    #[inline]
    fn invert(&self, coordinate: &Coord<T>) -> Coord<T> {
        forward_rotation_lambda(-self.delta_lambda, coordinate)
    }
}
