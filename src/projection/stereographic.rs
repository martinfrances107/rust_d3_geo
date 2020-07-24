use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::Transform;

use super::geo_projection::GeoProjection;
use super::geo_projection_mutator::GeoProjectionMutator;

// TODO must find a standard way to multiply by 2
fn angle<T>(z: T) -> T
where
  T: Float + FromPrimitive,
{
  return T::from_u8(2u8).unwrap() * z.atan();
}

struct StereographicRaw {}

impl<T> Transform<T> for StereographicRaw
where
  T: Float,
{
  fn transform(&self, &p: &[T; 2]) -> [T; 2] {
    let x = p[0];
    let y = p[1];
    let cy = y.cos();
    let k = T::one() + x.cos() * cy;
    return [cy * x.sin() / k, y.sin() / k];
  }

  fn invert(&self, p: &[T; 2]) -> [T; 2] {
    let x = p[0];
    let y = p[1];
    let z = (x * x + y * y).sqrt();
    let c = angle(z);
    let sc = c.sin();
    let cc = c.cos();
    return [(x * sc).atan2(z * cc), (y * sc / z).asin()];
  }
}

pub fn stereographic<T>() -> GeoProjectionMutator<T> {
  let s = Box::new(StereographicRaw {});
  let mut projection = GeoProjectionMutator::from_projection_raw(s);
  projection.scale(250f64);
  projection.clip_angle(Some(142f64));
  return projection;
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  // use super::*;

  // #[test]
  // fn test_stereographic_unit() {
  //   let s = StereographicRaw{};

  //   assert_eq!(s.transform(&[  0f64,   0f64]), [ 0f64,  0f64]);
  //   assert_eq!(s.transform(&[-90f64,   0f64]), [-1f64,  0f64]);
  //   assert_eq!(s.transform(&[ 90f64,   0f64]), [ 1f64,  0f64]);
  //   assert_eq!(s.transform(&[  0f64, -90f64]), [ 0f64,  1f64]);
  //   assert_eq!(s.transform(&[  0f64,  90f64]), [ 0f64, -1f64]);

  // }

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
