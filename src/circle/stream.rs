use std::cell::RefCell;
use std::rc::Rc;

use geo::Coordinate;
use num_traits::Float;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::rotation::rotate_radians::RotateRadians;
use crate::transform_stream::TransformStream;
use crate::Transform;
pub struct Stream<T: Float> {
    rotate: Rc<Box<dyn Transform<T>>>,
    ring: Rc<RefCell<Vec<Coordinate<T>>>>,
}

impl<T: Float + 'static> Stream<T> {
    pub fn new(
        rotate: Rc<Box<dyn Transform<T>>>,
        ring: Rc<RefCell<Vec<Coordinate<T>>>>,
    ) -> Box<dyn TransformStream<T>> {
        let rotate = rotate.clone();
        let ring = ring.clone();
        return Box::new(Self { rotate, ring });
    }
}

impl<T: Float> TransformStream<T> for Stream<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let x_rotated = self.rotate.invert(&Coordinate { x, y });
        let x_rotated_deg = Coordinate {
            x: x_rotated.x.to_degrees(),
            y: x_rotated.y.to_degrees(),
        };
        let mut ring = self.ring.borrow_mut();
        ring.push(x_rotated_deg);
    }
}
