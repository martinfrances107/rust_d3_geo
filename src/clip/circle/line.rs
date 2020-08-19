use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::clip::PointsVisibleFn;
use crate::math::epsilon;
use crate::point_equal::point_equal;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;

use super::intersect::intersect;
use super::intersect::Return;

// Takes a line and cuts into visible segments. Return values used for polygon
// clipPIng: 0 - there were intersections or the line was empty; 1 - no
// intersections 2 - there were intersections, and the first and last segments
// should be rejoined.


pub struct Line<F>
where
  F: Float,
{
  c0: u8,    // code for previous point
  clean: u8, // no intersections
  radius: F,
  rc: F,
  not_hemisphere: bool,
  point0: (Option<[F; 2]>, Option<u8>), // previous point with message.
  small_radius: bool,
  stream: Rc<RefCell<Box<dyn TransformStream<F>>>>,
  v0: bool,  // visibility of previous point
  v00: bool, // visibility of first point
  visible: Rc<PointsVisibleFn<F>>,
}

impl<F> Line<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  pub fn new(visible: Rc<PointsVisibleFn<F>>, radius: F) -> StreamProcessor<F> {
    return Box::new(
      move |stream_ptr: Rc<RefCell<Box<dyn TransformStream<F>>>>| {
        let stream = stream_ptr.clone();
        // TODO small_radius, rc  is a shadow variables!!!
        let rc = radius.cos();
        let small_radius = rc > F::zero();
        return Rc::new(RefCell::new(Box::new(Line::<F> {
          c0: 0,
          clean: 0,
          not_hemisphere: rc.abs() > epsilon(),
          point0: (None, None),
          rc,
          radius,
          small_radius,
          v0: false,
          v00: false,
          stream,
          visible: visible.clone(),
        })));
      },
    );
  }

  /// Generates a 4-bit vector representing the location of a point relative to
  /// the small circle's bounding box.
  fn code(&self, lambda: F, phi: F) -> u8 {
    let r = match self.small_radius {
      true => self.radius,
      false => F::PI() - self.radius,
    };
    let mut code = 0;
    if lambda < -r {
      code |= 1;
    }
    // left
    else if lambda > r {
      code |= 2;
    } // right
    if phi < -r {
      code |= 4;
    }
    // below
    else if phi > r {
      code |= 8;
    } // above
    return code;
  }

  /// Rejoin first and last segments if there were intersections and the first
  /// and last points were visible.
  fn clean(&self) -> u8 {
    return self.clean | (((self.v00 && self.v0) as u8) << 1);
  }
}

impl<F> TransformStream<F> for Line<F>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  fn line_start(&mut self) {
    self.v00 = false;
    self.v0 = false;
    self.clean = 1;
  }

  fn point(&mut self, lambda: F, phi: F, m: Option<u8>) {
    let mut point1: (Option<[F; 2]>, Option<u8>) = (Some([lambda, phi]), m);

    // let point2: (Option::<[F; 2]>, <Option<u8>>);
    let mut point2: (Option<[F; 2]>, Option<u8>);
    let v = (self.visible)(lambda, phi, None);

    let c = match self.small_radius {
      true => match v {
        true => 0u8,
        false => self.code(lambda, phi),
      },
      false => match v {
        true => {
          let inc = match lambda < F::zero() {
            true => F::PI(),
            false => -F::PI(),
          };
          self.code(lambda + inc, phi)
        }
        false => 0u8,
      },
    };

    if self.point0.0.is_none() {
      self.v00 = v;
      self.v0 = v;
      if v {
        let mut stream = self.stream.borrow_mut();
        stream.line_start();
      }
    }

    if v != self.v0 {
      // _r reduced from 3d to 2d.
      match intersect(
        self.point0.0.unwrap(),
        point1.0.unwrap(),
        self.radius.cos(),
        false,
      ) {
        Return::None => {
          point2 = (None, None);
        }
        Return::One(p) => {
          point2 = (Some(p), None);
        }
        Return::Two(_t) => {
          panic!("requested One or None found Two as !!");
        }
      }
      if point2.1.is_none()
        || point_equal(self.point0.0.unwrap(), point2.0.unwrap())
        || point_equal(point1.0.unwrap(), point2.0.unwrap())
      {
        point1.1 = Some(1u8);
      }
    }

    let mut stream = self.stream.borrow_mut();
    if v != self.v0 {
      self.clean = 0;
      if v {
        // outside going in
        stream.line_start();
        match intersect(point1.0.unwrap(), self.point0.0.unwrap(), self.rc, false) {
          Return::None => {
            point2 = (None, None);
          }
          Return::One(p) => {
            point2 = (Some(p), None);
          }
          Return::Two(_) => {
            panic!("requested One or None found Two!!");
          }
        }

        stream.point(point2.0.unwrap()[0], point2.0.unwrap()[1], None);
      } else {
        // inside going out
        // point2 = intersect(self.point0.unwrap(), point1.unwrap(), self.rc, false);
        match intersect(self.point0.0.unwrap(), point1.0.unwrap(), self.rc, false) {
          Return::None => {
            point2 = (None, None);
          }
          Return::One(p) => {
            point2 = (Some(p), None);
          }
          Return::Two(_) => {
            panic!("requested One or None found Two!!");
          }
        }

        stream.point(point2.0.unwrap()[0], point2.0.unwrap()[1], Some(2u8));
        stream.line_end();
        self.point0 = point2;
      }
    } else if self.not_hemisphere && self.point0.0.is_none() && self.small_radius ^ v {
      // If the codes for two points are different, or are both zero,
      // and there this segment intersects with the small circle.
      if (c & self.c0) != 0 {
        let t = intersect(point1.0.unwrap(), self.point0.0.unwrap(), self.rc, true);
        match t {
          Return::None => {}
          Return::One(_) => {
            panic!("requetsed two received one");
          }
          Return::Two(t) => {
            self.clean = 0;
            if self.small_radius {
              stream.line_start();
              stream.point(t[0][0], t[0][1], None);
              stream.point(t[1][0], t[1][1], None);
              stream.line_end();
            } else {
              stream.point(t[1][0], t[1][1], None);
              stream.line_end();
              stream.line_start();
              stream.point(t[0][0], t[0][1], Some(3u8));
            }
          }
        }
      }
    }
    if v && self.point0.0.is_none() || !point_equal(self.point0.0.unwrap(), point1.0.unwrap()) {
      stream.point(point1.0.unwrap()[0], point1.0.unwrap()[1], None);
    }
    self.point0 = point1;
    self.v0 = v;
    self.c0 = c;
  }

  fn line_end(&mut self) {
    if self.v0 {
      let mut stream = self.stream.borrow_mut();
      stream.line_end();
    }
    self.point0 = (None, None);
  }
}
// function clipLine(stream) {
//   var point0, // previous point
//       c0, // code for previous point
//       v0, // visibility of previous point
//       v00, // visibility of first point
//       clean; // no intersections
//   return {
//     lineStart: function() {
//       v00 = v0 = false;
//       clean = 1;
//     },
//     point: function(lambda, phi) {
//       var point1 = [lambda, phi],
//           point2,
//           v = visible(lambda, phi),
//           c = smallRadius
//             ? v ? 0 : code(lambda, phi)
//             : v ? code(lambda + (lambda < 0 ? PI : -PI), phi) : 0;
//       if (!point0 && (v00 = v0 = v)) stream.lineStart();
//       if (v !== v0) {
//         point2 = intersect(point0, point1);
//         if (!point2 || pointEqual(point0, point2) || pointEqual(point1, point2))
//           point1[2] = 1;
//       }
//       if (v !== v0) {
//         clean = 0;
//         if (v) {
//           // outside going in
//           stream.lineStart();
//           point2 = intersect(point1, point0);
//           stream.point(point2[0], point2[1], None);
//         } else {
//           // inside going out
//           point2 = intersect(point0, point1);
//           stream.point(point2[0], point2[1], 2);
//           stream.lineEnd();
//         }
//         point0 = point2;
//       } else if (notHemisphere && point0 && smallRadius ^ v) {
//         var t;
//         // If the codes for two points are different, or are both zero,
//         // and there this segment intersects with the small circle.
//         if (!(c & c0) && (t = intersect(point1, point0, true))) {
//           clean = 0;
//           if (smallRadius) {
//             stream.lineStart();
//             stream.point(t[0][0], t[0][1], None);
//             stream.point(t[1][0], t[1][1], None);
//             stream.lineEnd();
//           } else {
//             stream.point(t[1][0], t[1][1], None);
//             stream.lineEnd();
//             stream.lineStart();
//             stream.point(t[0][0], t[0][1], 3);
//           }
//         }
//       }
//       if (v && (!point0 || !pointEqual(point0, point1))) {
//         stream.point(point1[0], point1[1], None);
//       }
//       point0 = point1, v0 = v, c0 = c;
//     },
//     lineEnd: function() {
//       if (v0) stream.lineEnd();
//       point0 = null;
//     },
//     // Rejoin first and last segments if there were intersections and the first
//     // and last points were visible.
//     clean: function() {
//       return clean | ((v00 && v0) << 1);
//     }
//   };
// }
