// use crate::types::MapTransform;

// pub fn azimuthal_raw(scale: fn(scale: f64) -> f64) -> MapTransform {
//   return Box::new(|p: [f64; 2]| -> [f64; 2] {
//     let x = p[0];
//     let y = p[1];
//     let cx = x.cos();
//     let cy = y.cos();
//     let k = scale(cx * cy);

//     return match k.is_infinite() {
//       true => [2f64, 0f64],
//       false => [k * cy * x.sin(), k * y.sin()],
//     };
//   });
// }

// // pub fn azimuthal_invert(angle: Box<dyn Fn(f64) -> f64>) -> MapTransform {

// //     let x = p[0];
// //     let y = p[1];
// //     let z = (x * x + y * y).sqrt();const
// //     let c = angle(z);
// //     let sc = c.sin();
// //     let cc = c.cos();
// //     return [(x * sc).atan2(z * cc), (y * sc / z).asin()];

// // }

// static  aximuthal_invert: Box<dyn Fn() -> [f64;2]> = Fn(p[f64:2] )  {
//   let x = p[0];
//   let y = p[1];
//   let z = (x * x + y * y).sqrt();
//   let c = angle(z);
//   let sc = c.sin();
//   let cc = c.cos();
//   return [(x * sc).atan2(z * cc), (y * sc / z).asin()];

// }
