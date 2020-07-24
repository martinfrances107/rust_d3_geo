use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::math::EPSILON;
use crate::Transform;

use crate::projection::geo_stream::GeoStream;

// import {cartesian} from "../cartesian.js";
// import {transformer} from "../transform.js";

const MAXDEPTH:u8 = 16u8; // maximum depth of subdivision

// export default function(project, delta2) {
//   return +delta2 ? resample(project, delta2) : resampleNone(project);
// }

// function resampleNone(project) {
//   return transformer({
//     point: function(x, y) {
//       x = project(x, y);
//       this.stream.point(x[0], x[1]);
//     }
//   });
// }

pub struct Resample<T> {
  project: Box<dyn Transform<T>>,
  delta2: T,

  lambda00:T,
  x00:T,
  y00:T,
  a00:T,
  b00:T,
  c00:T, // first point

  lambda0:T,
  x0:T,
  y0:T,
  a0:T,
  b0:T,
  c0:T, // previous point

  cos_min_distance: T,

}

impl<T> Resample<T> {

  pub fn new(project:Box<dyn Transform<T>>, delta2: T) -> Self
  where T: Float + FromPrimitive {
    return Self{
      project,
      delta2,

      lambda00:T::zero(),
      x00:T::zero(),
      y00:T::zero(),
      a00:T::zero(),
      b00:T::zero(),
      c00:T::zero(), // first point

      lambda0:T::zero(),
      x0:T::zero(),
      y0:T::zero(),
      a0:T::zero(),
      b0:T::zero(),
      c0:T::zero(), // previous point
      cos_min_distance: (30f64 * RADIANS).cos() // cos(minimum angular distance)
    };
  }

  fn resample_line_to(self,x0:f64, y0:f64, lambda0:f64, a0:f64, b0:f64, c0:f64, x1:f64, y1:f64, lambda1:f64, a1:f64, b1:f64, c1:f64, depth_p:u8, stream: Box<dyn GeoStream>) {
    let mut depth = depth_p;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let d2 = dx * dx + dy * dy;

    // if (d2 > 4 * delta2 && depth--) {
    if d2 > 4f64 * self.delta2  {
      depth = depth - 1u8;
      if depth > 0u8 {
        let mut a = a0 + a1;
        let mut b = b0 + b1;
        let mut c:f64 = c0 + c1;
        let m:f64 = (a * a + b * b + c * c).sqrt();
        c = c / m;
        let phi2 = c.asin();
        let lambda2 = match ((c.abs() - 1f64).abs() < EPSILON, (lambda0 - lambda1).abs() < EPSILON) {
          (true, _) | (_, true)   => {(lambda0 + lambda1) / 2f64},
          (false, false) => {b.atan2(a)}
        };
        //  let lambda2 = abs(abs(c) - 1) < EPSILON || abs(lambda0 - lambda1) < EPSILON ? (lambda0 + lambda1) / 2 : b.atan2(a),
        let p = self.project.transform(&[lambda2, phi2]);
        let x2 = p[0];
        let y2 = p[1];
        let dx2 = x2 - x0;
        let dy2 = y2 - y0;
        let dz = dy * dx2 - dx * dy2;
        // Three condtions :-
        // perpendicular projected distance
        // midpoint close to an end
        // angular distance
        if dz * dz / d2 > self.delta2 ||
           ((dx * dx2 + dy * dy2) / d2 - 0.5f64).abs() > 0.3f64 ||
           a0 * a1 + b0 * b1 + c0 * c1 < self.cos_min_distance {
          a = a / m;
          b = b /m;
          // self.resample_line_to(x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b, c, depth, stream);
          // stream.point(x2, y2);
          // self.resample_line_to(x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth, stream);
        }

    }
  }
}

}