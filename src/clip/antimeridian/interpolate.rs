use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::math::epsilon;

use crate::transform_stream::TransformStream;

pub fn interpolate<F>(
  from: Option<[F; 2]>,
  to: Option<[F; 2]>,
  direction: F,
  stream: Rc<RefCell<Box<dyn TransformStream<F>>>>,
) where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  let phi: F;
  let mut stream = stream.borrow_mut();
  if from.is_none() {
    phi = direction * F::FRAC_PI_2();
    stream.point(-F::PI(), phi, None);
    stream.point(F::zero(), phi, None);
    stream.point(F::PI(), phi, None);
    stream.point(F::PI(), F::zero(), None);
    stream.point(F::PI(), -phi, None);
    stream.point(F::zero(), -phi, None);
    stream.point(-F::PI(), -phi, None);
    stream.point(-F::PI(), F::zero(), None);
    stream.point(-F::PI(), phi, None);
  } else if (from.unwrap()[0] - to.unwrap()[0]).abs() > epsilon() {
    let lambda = if from.unwrap()[0] < to.unwrap()[0] {
      F::PI()
    } else {
      -F::PI()
    };
    let f_2 = F::from(2u8).unwrap();
    phi = direction * lambda / f_2;
    stream.point(-lambda, phi, None);
    stream.point(F::zero(), phi, None);
    stream.point(lambda, phi, None);
  } else {
    stream.point(to.unwrap()[0], to.unwrap()[1], None);
  }
}
