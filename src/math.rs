use std::f64::consts::PI;

pub const EPSILON:f64 = 1e-6;
pub const EPSILON2:f64 = 1e-12;
// pub const PI: f32 = Math.PI
pub const HALFPI:f64  = PI / 2f64;
pub const QUATERPI: f64 = PI / 4f64;

///  std::f64::consts::TAU; is valid by in a unstable release!
pub const TAU: f64 = PI * 2f64;

pub const DEGREES: f64 = 180f64 / PI;
pub const RADIANS: f64 = PI / 180f64;
// pub coMathnst ceil = Math.ceil;

// see f64.signum()
// pub const sign =
//   Math.sign ||
//   function(x) {
//     return x > 0 ? 1 : x < 0 ? -1 : 0;
//   };

// export var sqrt = Math.sqrt;
// export var tan = Math.tan;

// export function acos(x) {
//   return x > 1 ? 0 : x < -1 ? PI : Math.acos(x);
// }

// export function asin(x) {
//   return x > 1 ? halfPI : x < -1 ? -halfPI : Math.asin(x);
// }

// export function haversin(x) {
pub fn haversin(x:f64) -> f64{
  let sinxdiv2: f64 = (x / 2f64).sin();
  return sinxdiv2 * sinxdiv2;
}
