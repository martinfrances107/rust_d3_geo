use std::cell::RefCell;
use std::rc::Rc;

use delaunator::Point;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::rotation::rotate_radians::RotateRadians;
use crate::transform_stream::TransformStream;
use crate::Transform;
pub struct Stream
{
  rotate: Rc<Box<dyn Transform>>,
  ring: Rc<RefCell<Vec<Point>>>,
}

impl Stream
{
  pub fn new(
    rotate: Rc<Box<dyn Transform>>,
    ring: Rc<RefCell<Vec<Point>>>,
  ) -> Box<dyn TransformStream>
  {
    let rotate = rotate.clone();
    let ring = ring.clone();
    return Box::new(Self { rotate, ring });
  }
}

impl TransformStream for Stream
{
  fn point(&mut self, x: f64, y: f64, m: Option<u8>)
  {
    let x_rotated = self.rotate.invert(&Point{x, y});
    let x_rotated_deg = Point{x:x_rotated.x.to_degrees(), y:x_rotated.y.to_degrees()};
    let mut ring = self.ring.borrow_mut();
    ring.push(x_rotated_deg);
  }
}
