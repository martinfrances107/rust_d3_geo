// use delaunator::Point;
use geo::Point;
use num_traits::{float::Float, FloatConst};

use super::rotate_radians::RotateRadians;
use crate::Transform;

pub struct Rotation<T> {
    rotate: Box<dyn Transform<T>>,
}

impl<T: Float + FloatConst + 'static> Rotation<T> {
    pub fn new(delta_lambda: T, delta_phi: T, delta_gamma: T) -> Self {
        return Self {
            rotate: RotateRadians::new(
                delta_lambda.to_radians(),
                delta_phi.to_radians(),
                delta_gamma.to_radians(),
            ),
        };
    }
}

impl<T: Float> Transform<T> for Rotation<T> {
    fn transform(&self, coordinates: &Point<T>) -> Point<T> {
        let temp = self.rotate.transform(&Point::new(
            coordinates.x().to_radians(),
            coordinates.y().to_radians(),
        ));
        return Point::new(temp.x().to_degrees(), temp.y().to_degrees());
    }

    fn invert(&self, coordinates: &Point<T>) -> Point<T> {
        let temp = self.rotate.invert(&Point::new(
            coordinates.x().to_radians(),
            coordinates.y().to_radians(),
        ));
        return Point::new(temp.x().to_degrees(), temp.y().to_degrees());
    }
}
