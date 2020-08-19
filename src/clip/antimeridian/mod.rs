use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

mod interpolate;
mod intersect;
mod line;

use crate::transform_stream::StreamProcessor;

use super::Clip;

use interpolate::interpolate;
use line::Line;

pub fn point_visible<F>(_x: F, _y: F, _z: Option<F>) -> bool
where
  F: Float + FromPrimitive,
{
  return true;
}

pub fn generate_antimeridian<F>() -> StreamProcessor<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  let cal: StreamProcessor<F> = Line::new();

  let clip_line_fn_ptr: Rc<RefCell<StreamProcessor<F>>>;
  clip_line_fn_ptr = Rc::new(RefCell::new(cal));

  return Clip::<F>::new(
    Rc::new(Box::new(point_visible)),
    clip_line_fn_ptr,
    Rc::new(RefCell::new(Box::new(interpolate::<F>))),
    [-F::PI(), -F::FRAC_PI_2()],
  );
}
