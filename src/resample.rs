use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

// use crate::math::EPSILON;
use crate::Transform;

use crate::stream::GeoStream;

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

pub struct Resample<F> {
  project: Box<dyn Transform<F>>,
  delta2: F,

  lambda00:F,
  x00:F,
  y00:F,
  a00:F,
  b00:F,
  c00:F, // first point

  lambda0:F,
  x0:F,
  y0:F,
  a0:F,
  b0:F,
  c0:F, // previous point

  cos_min_distance: F,

}

impl<F> Resample<F> {
  pub fn new(project:Box<dyn Transform<F>>, delta2: F) -> Self
  where F: Float + FromPrimitive + FloatConst{
    return Self{
      project,
      delta2,

      lambda00:F::zero(),
      x00:F::zero(),
      y00:F::zero(),
      a00:F::zero(),
      b00:F::zero(),
      c00:F::zero(), // first point

      lambda0:F::zero(),
      x0:F::zero(),
      y0:F::zero(),
      a0:F::zero(),
      b0:F::zero(),
      c0:F::zero(), // previous point
      cos_min_distance: (F::from(30u8).unwrap().to_radians()).cos() // cos(minimum angular distance)
    };
  }

  fn resample_line_to(self,x0:F, y0:F, lambda0:F, a0:F, b0:F, c0:F, x1:F, y1:F, lambda1:F, a1:F, b1:F, c1:F, depth_p:u8, stream: Box<dyn GeoStream::<F>>)
  where F: Float + FloatConst {
    let mut depth = depth_p;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let d2 = dx * dx + dy * dy;
    let float_4 = F::from(4u8).unwrap();
    let float_2 = F::from(2u8).unwrap();
    // if (d2 > 4 * delta2 && depth--) {
    if d2 > float_4 * self.delta2  {
      depth = depth - 1u8;
      if depth > 0u8 {
        let mut a = a0 + a1;
        let mut b = b0 + b1;
        let mut c:F = c0 + c1;
        let m:F = (a * a + b * b + c * c).sqrt();
        c = c / m;
        let phi2 = c.asin();
        let lambda2 = match ((c.abs() - F::one()).abs() < Float::epsilon(), (lambda0 - lambda1).abs() < F::epsilon()) {
          (true, _) | (_, true)   => {(lambda0 + lambda1) / float_2},
          (false, false) => {b.atan2(a)}
        };
        let f_2 = F::from(2u8).unwrap();
        let lambda2 = match  (c.abs() - F::one()).abs() < F::epsilon() || (lambda0 - lambda1).abs() < F::epsilon() {
          true =>  (lambda0 + lambda1) / f_2,
          false =>  b.atan2(a),
        };
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
        // TODO must find a way to make this constants static
        let float_1_2 = F::from(0.5f64).unwrap();
        let float_1_3 = F::from(0.3f64).unwrap();
        if dz * dz / d2 > self.delta2 ||
           ((dx * dx2 + dy * dy2) / d2 - float_1_2).abs() > float_1_3 ||
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