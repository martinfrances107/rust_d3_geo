use std::cell::RefCell;
use std::rc::Rc;

use geo::Point;
use num_traits::Float;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::rotation::rotate_radians::RotateRadians;
use crate::transform_stream::TransformStream;
use crate::Transform;
pub struct Stream<T: Float> {
    rotate: Rc<Box<dyn Transform<T>>>,
    ring: Rc<RefCell<Vec<Point<T>>>>,
}

impl<T: Float + 'static> Stream<T> {
    pub fn new(
        rotate: Rc<Box<dyn Transform<T>>>,
        ring: Rc<RefCell<Vec<Point<T>>>>,
    ) -> Box<dyn TransformStream<T>> {
        let rotate = rotate.clone();
        let ring = ring.clone();
        return Box::new(Self { rotate, ring });
    }
}

impl<T: Float> TransformStream<T> for Stream<T> {
    fn point(&mut self, x: T, y: T, m: Option<u8>) {
        let x_rotated = self.rotate.invert(&Point::new(x, y));
        let x_rotated_deg = Point::new(x_rotated.x().to_degrees(), x_rotated.y().to_degrees());
        let mut ring = self.ring.borrow_mut();
        ring.push(x_rotated_deg);
    }
}
