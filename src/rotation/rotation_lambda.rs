use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use crate::math::TAU;
use crate::Transform;
use crate::TransformClone;

#[derive(Clone, Copy, Default, Debug)]
pub struct RotationLambda<T> {
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

impl<T: CoordFloat + FloatConst + 'static> RotationLambda<T> {
    #[inline]
    pub fn new(delta_lambda: T) -> Box<dyn Transform<TcC = Coordinate<T>>> {
        Box::new(Self { delta_lambda })
    }
}

impl<T: CoordFloat + FloatConst + 'static> TransformClone for RotationLambda<T> {
    type TcC = Coordinate<T>;
    fn box_clone(&self) -> Box<dyn Transform<TcC = Self::TcC>> {
        Box::new(self.clone())
    }
}

impl<T: CoordFloat + FloatConst + 'static> Transform for RotationLambda<T> {
    #[inline]
    fn transform(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        forward_rotation_lambda(self.delta_lambda, coordinates)
    }
    #[inline]
    fn invert(&self, coordinates: &Coordinate<T>) -> Coordinate<T> {
        forward_rotation_lambda(-self.delta_lambda, coordinates)
    }
}
