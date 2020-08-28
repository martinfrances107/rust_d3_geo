pub mod rotate_radians;
pub mod rotation;

mod rotation_identity;
mod rotation_lambda;
mod rotation_phi_gamma;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::in_delta::in_delta;
  use crate::Transform;
  use rotation::Rotation;

  #[test]
  fn only_longitude() {
    println!("a rotation of [+90°, 0°] only rotates longitude");
    let rotation = Rotation::new(90f64, 0f64, 0f64).transform(&[0f64, 0f64]);
    assert!(in_delta(rotation[0], 90f64, 1e-6));
    assert!(in_delta(rotation[1], 0f64, 1e-6));
  }

  #[test]
  fn wraps_antimeridan() {
    println!("a rotation of [+90°, 0°] wraps around when crossing the antimeridian");
    let rotation = Rotation::new(90f64, 0f64, 0f64).transform(&[150f64, 0f64]);
    assert!(in_delta(rotation[0], -120f64, 1e-6));
    assert!(in_delta(rotation[1], 0f64, 1e-6));
  }

  #[test]
  fn rotation_long_and_lat() {
    println!("a rotation of [-45°, 45°] rotates longitude and latitude");
    let rotation = Rotation::new(-45f64, 45f64, 0f64).transform(&[0f64, 0f64]);
    assert!(in_delta(rotation[0], -54.73561f64, 1e-6));
    assert!(in_delta(rotation[1], 30f64, 1e-6));
  }

  #[test]
  fn rotation_inverse_long_lat() {
    println!("a rotation of [-45°, 45°] rotates longitude and latitude");
    let rotation = Rotation::new(-45f64, 45f64, 0f64).invert(&[-54.73561f64, 30f64]);
    assert!(in_delta(rotation[0], 0f64, 1e-6));
    assert!(in_delta(rotation[1], 0f64, 1e-6));
  }

  #[test]
  fn identity_rotation() {
    println!("the identity rotation constrains longitudes to [-180°, 180°]");
    let rotate = Rotation::new(0f64, 0f64, 0f64);
    assert_eq!(rotate.transform(&[180f64, 0f64])[0], 180f64);
    assert_eq!(rotate.transform(&[-180f64, 0f64])[0], -180f64);
    assert_eq!(rotate.transform(&[360f64, 0f64])[0], 0f64);
    assert!(in_delta(
      rotate.transform(&[2562f64, 0f64])[0],
      42f64,
      1e-10
    ));
    assert!(in_delta(
      rotate.transform(&[-2562f64, 0f64])[0],
      -42f64,
      1e-10
    ));
  }
}
