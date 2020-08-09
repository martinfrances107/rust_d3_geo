use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

mod interpolate;
mod intersect;
mod line;

// use crate::stream::GeoStream;
// use crate::transform_stream::TransformStream;
// use crate::transform_stream::TransformStreamIdentity;

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
pub fn generate_antimeridian<F>() -> Clip<F>
where
  F: Float + FloatConst + FromPrimitive + 'static
{
  return Clip::<F>::new(
    Box::new(point_visible),
     Box::new(ClipAntimeridianLine::new()),
     Box::new(Interpolate::new()),
     [-F::PI(), -F::FRAC_PI_2()],
  );
}
