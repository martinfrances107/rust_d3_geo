use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

mod interpolate;
mod intersect;
mod line;

use crate::transform_stream::TransformStream;

use super::Clip;

use interpolate::Interpolate;
use line::ClipAntimeridianLine;

pub fn point_visible<F>(_x: F, _y: F, _z: Option<F>) -> bool
where
  F: Float + FromPrimitive,
{
  return true;
}

// export default clip(
//   function() { return true; },
//   clipAntimeridianLine,
//   clipAntimeridianInterpolate,use num_traits::FloatConst;
//   [-pi, -halfPi]
// );
pub fn generate_antimeridian<F>() -> Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Box<dyn TransformStream<F>>>
where
  F: Float + FloatConst + FromPrimitive + 'static
{
  let cal: Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Box<dyn TransformStream<F>>> = ClipAntimeridianLine::new();

  let clip_line_fn_ptr: Rc<RefCell<Box<dyn Fn(Rc<RefCell<Box<dyn TransformStream<F>>>>) -> Box<dyn TransformStream<F>>>>>;
  clip_line_fn_ptr = Rc::new(RefCell::new(Box::new(cal)));

  return Clip::<F>::new(
    Rc::new(Box::new(point_visible)),
    clip_line_fn_ptr,
    Rc::new(RefCell::new(Box::new(Interpolate::new()))),
    [-F::PI(), -F::FRAC_PI_2()],
  );
}
