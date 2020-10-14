use std::cell::RefCell;
use std::rc::Rc;
use std::f64;
use delaunator::Point;

// use crate::math::epsilon;

use crate::transform_stream::TransformStream;

pub fn interpolate(
  from: Option<Point>,
  to: Option<Point>,
  direction: f64,
  stream: Rc<RefCell<Box<dyn TransformStream>>>,
)
{
  let phi: f64;
  let mut stream = stream.borrow_mut();
  match from {
    None => {
    phi = direction * f64::consts::FRAC_PI_2;
    stream.point(-f64::consts::PI, phi, None);
    stream.point(0f64, phi, None);
    stream.point(f64::consts::PI, phi, None);
    stream.point(f64::consts::PI, 0f64, None);
    stream.point(f64::consts::PI, -phi, None);
    stream.point(0f64, -phi, None);
    stream.point(-f64::consts::PI, -phi, None);
    stream.point(-f64::consts::PI, 0f64, None);
    stream.point(-f64::consts::PI, phi, None);
    },
    Some(from) => {
      // TODO investigate is to and Option<f64>
      let to = to.unwrap();
      if (from.x - to.x).abs() > f64::EPSILON {
        let lambda = if from.x < to.x {
          f64::consts::PI
        } else {
          -f64::consts::PI
        };

        phi = direction * lambda / 2f64;
        stream.point(-lambda, phi, None);
        stream.point(0f64, phi, None);
        stream.point(lambda, phi, None);
      } else {
        stream.point(to.x, to.y, None);
      }
    }
  }

}
