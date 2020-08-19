mod intersect;
mod line;

use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use super::Clip;
use crate::clip::PointsVisibleFn;
use crate::circle::circle_stream::circle_stream;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use line::Line;

pub fn generate_circle<F>(radius: F) -> StreamProcessor<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  let cr = radius.cos();
  let delta = F::from(6u8).unwrap().to_radians();

  let visible: Rc<PointsVisibleFn<F>> = Rc::new(Box::new(move |lambda: F, phi: F, _m: Option<F>| {
    return lambda.cos() * phi.cos() > cr;
  }));

  let interpolate = move |from: Option<[F; 2]>,
                          to: Option<[F; 2]>,
                          direction: F,
                          stream: Rc<RefCell<Box<dyn TransformStream<F>>>>| {
    circle_stream(stream, radius, delta, direction, from, to)
  };

  let ccl: StreamProcessor<F> = Line::new(visible.clone(), radius);
  let clip_line_fn_ptr: Rc<RefCell<StreamProcessor<F>>>;
  clip_line_fn_ptr = Rc::new(RefCell::new(Box::new(ccl)));

  return Clip::<F>::new(
    visible,
    clip_line_fn_ptr,
    Rc::new(RefCell::new(Box::new(interpolate))),
    [-F::PI(), -F::FRAC_PI_2()],
  );
}
