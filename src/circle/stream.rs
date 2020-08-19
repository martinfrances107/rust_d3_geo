use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::rotation::rotate_radians::RotateRadians;
use crate::transform_stream::TransformStream;
use crate::Transform;

pub struct Stream<F>
where
  F: Float,
{
  rotate: Rc<Box<dyn Transform<F>>>,
  ring: Rc<RefCell<Vec<[F; 2]>>>,
}

impl<F> Stream<F>
where
  F: Float + FloatConst,
{
  pub fn new(
    rotate: Rc<Box<dyn Transform<F>>>,
    ring: Rc<RefCell<Vec<[F; 2]>>>,
  ) -> Box<dyn TransformStream<F>>
  where
    F: Float + 'static,
  {
    let rotate = rotate.clone();
    let ring = ring.clone();
    return Box::new(Self { rotate, ring });
  }
}

impl<F> TransformStream<F> for Stream<F>
where
  F: Float + FloatConst + 'static,
{
  fn point(&mut self, x: F, y: F, m: Option<u8>)
  where
    F: Float + FloatConst + 'static,
  {
    let x_rotated = self.rotate.invert(&[x, y]);
    let x_rotated_deg = [x_rotated[0].to_degrees(), x_rotated[1].to_degrees()];
    let mut ring = self.ring.borrow_mut();
    ring.push(x_rotated_deg);
  }
}
