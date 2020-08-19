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

// import {cartesian, cartesianAddInPlace, cartesianCross, cartesianDot, cartesianScale, spherical} from "../cartesian.js";
// import {circleStream} from "../circle.js";
// import {abs, cos, epsilon, PI, radians, sqrt} from "../math.js";
// import pointEqual from "../pointEqual.js";
// import clip from "./index.js";

// export default function(radius) {
//   var cr = cos(radius),
//       delta = 6 * radians,
//       smallRadius = cr > 0,
//       notHemisphere = abs(cr) > epsilon; // TODO optimise for this common case

//   function visible(lambda, phi) {
//     return cos(lambda) * cos(phi) > cr;
//   }

//   // Generates a 4-bit vector representing the location of a point relative to
//   // the small circle's bounding box.
//   function code(lambda, phi) {
//     var r = smallRadius ? radius : PI - radius,
//         code = 0;
//     if (lambda < -r) code |= 1; // left
//     else if (lambda > r) code |= 2; // right
//     if (phi < -r) code |= 4; // below
//     else if (phi > r) code |= 8; // above
//     return code;
//   }

//   return clip(visible, clipLine, interpolate, smallRadius ? [0, -radius] : [-PI, radius - PI]);
// }
