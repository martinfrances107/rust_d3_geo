use geo::Point;
use num_traits::Float;
use num_traits::FloatConst;

// use crate::math::TAU;
use crate::Transform;

#[derive(Debug)]
pub struct RotationLambda<T> {
    pub delta_lambda: T,
}

// TODO why can't I #[inline] this.
fn forward_rotation_lambda<T: Float + FloatConst>(delta_lambda: T, p: &Point<T>) -> Point<T> {
    let lambda = p.x() + delta_lambda;
    let phi = p.y();
    return match (lambda > T::PI(), lambda < -T::PI()) {
        (false, false) => Point::new(lambda, phi), // -PI <= lambda <= PI
        (true, _) => Point::new(lambda - T::TAU(), phi), // lambda >  PI
        (_, true) => Point::new(lambda + T::TAU(), phi), // lambda < -PI
    };
}

impl<T: Float + FloatConst + 'static> RotationLambda<T> {
    pub fn new(delta_lambda: T) -> Box<dyn Transform<T>> {
        return Box::new(Self { delta_lambda });
    }
}

impl<T: Float + FloatConst> Transform<T> for RotationLambda<T> {
    fn transform(&self, coordinates: &Point<T>) -> Point<T> {
        return forward_rotation_lambda(self.delta_lambda, coordinates);
    }
    fn invert(&self, coordinates: &Point<T>) -> Point<T> {
        return forward_rotation_lambda(-self.delta_lambda, coordinates);
    }
}
