use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

use super::projection::Projection;
use super::projection_mutator::ProjectionMutator;

// TODO must find a standard way to multiply by 2
fn angle<F>(z: F) -> F
where
  F: Float + FromPrimitive,
{
  return F::from_u8(2u8).unwrap() * z.atan();
}

#[derive(Clone, Debug)]
struct StereographicRaw {}

impl StereographicRaw {
  fn new<F>() -> Box<dyn Transform<F>>
  where
    F: Float + FloatConst + FromPrimitive,
  {
    return Box::new(StereographicRaw {});
  }

  pub fn gen_projection_mutator<'a, F>() -> ProjectionMutator<F>
  where
    F: Float + FloatConst + FromPrimitive + 'static
  {
    let s = StereographicRaw::new();
    let mut projection = ProjectionMutator::from_projection_raw(s);
    projection.scale(&F::from(250u8).unwrap());
    projection.clip_angle(Some(F::from_u8(142u8).unwrap()));
    return projection;
  }
}

impl<F> Transform<F> for StereographicRaw
where
  F: Float + FloatConst + FromPrimitive,
{
  fn transform(&self, &p: &[F; 2]) -> [F; 2] {
    let x = p[0];
    let y = p[1];
    let cy = y.cos();
    let k = F::one() + x.cos() * cy;
    return [cy * x.sin() / k, y.sin() / k];
  }

  fn invert(&self, p: &[F; 2]) -> [F; 2] {
    let x = p[0];
    let y = p[1];
    let z = (x * x + y * y).sqrt();
    let c = angle(z);
    let sc = c.sin();
    let cc = c.cos();
    return [(x * sc).atan2(z * cc), (y * sc / z).asin()];
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_stereographic_embedded() {
    let mut stereo = StereographicRaw::gen_projection_mutator();
    stereo.translate(&[0f64, 0f64]);
    stereo.scale(&1f64);

    // assert_eq!(stereo.projection.transform(&[  0f64,   0f64]), [ 0f64,  0f64]);
    // assert_eq!(stereo.projection.transform(&[-90f64,   0f64]), [-1f64,  0f64]);
    // assert_eq!(stereo.projection.transform(&[ 90f64,   0f64]), [ 1f64,  0f64]);
    // assert_eq!(stereo.projection.transform(&[  0f64, -90f64]), [ 0f64,  1f64]);
    // assert_eq!(stereo.projection.transform(&[0f64, 90f64]), [0f64, -1f64]);
  }

  // #[test]
  // fn test_stereographic_embedded() {
  //   let mut s = stereographic();
  //   s.translate([0f64, 0f64]);
  //   s.scale(1f64);

  //   assert_eq!(s.projection.transform(&[  0f64,   0f64]), [ 0f64,  0f64]);
  //   assert_eq!(s.projection.transform(&[-90f64,   0f64]), [-1f64,  0f64]);
  //   assert_eq!(s.projection.transform(&[ 90f64,   0f64]), [ 1f64,  0f64]);
  //   assert_eq!(s.projection.transform(&[  0f64, -90f64]), [ 0f64,  1f64]);
  //   assert_eq!(s.projection.transform(&[  0f64,  90f64]), [ 0f64, -1f64]);

  // }
}
