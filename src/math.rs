use num_traits::cast::FromPrimitive;
use num_traits::Float;


pub fn epsilon<F>() -> F
where F: Float + FromPrimitive {
  return  F::from(1e-6).unwrap();
}

// pub const EPSILON2:f64 = 1e-12;

// pub coMathnst ceil = Math.ceil;

// see f64.signum()
// pub const sign =
//   Math.sign ||
//   function(x) {
//     return x > 0 ? 1 : x < 0 ? -1 : 0;
//   };


// export function acos(x) {
//   return x > 1 ? 0 : x < -1 ? PI : Math.acos(x);
// }

// export function asin(x) {
//   return x > 1 ? halfPI : x < -1 ? -halfPI : Math.asin(x);
// }

// export function haversin(x) {
// pub fn haversin(x:f64) -> f64{
//   let sinxdiv2: f64 = (x / 2f64).sin();
//   return sinxdiv2 * sinxdiv2;
// }
