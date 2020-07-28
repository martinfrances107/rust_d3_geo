use num_traits::Float;
use num_traits::FloatConst;

mod line;
mod interpolate;
mod intersect;

use crate::stream::GeoStream;

use super::Clip;

use line::ClipAntimeridianLine;
use intersect::intersect;
use interpolate::interpolate;

fn point_visible() -> bool { true }


// export default clip(
//   function() { return true; },
//   clipAntimeridianLine,
//   clipAntimeridianInterpolate,
//   [-pi, -halfPi]
// );
pub fn generate_antimeridian<F>(sink: Box<dyn GeoStream<F>>)-> Clip<F>
where F: Float + FloatConst {
  return Clip::new(
    point_visible,
    ClipAntimeridianLine::<F>::new(sink),
    interpolate::<F>,
    [-F::PI(), -F::FRAC_PI_2()],
    sink
  );
}

