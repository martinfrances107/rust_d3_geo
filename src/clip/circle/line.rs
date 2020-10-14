use std::cell::RefCell;
use std::f64;
use std::rc::Rc;

use delaunator::Point;

use crate::clip::PointsVisibleFn;
use crate::point_equal::point_equal;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;

use super::intersect::intersect;
use super::intersect::IntersectReturn;

// Takes a line and cuts into visible segments. Return values used for polygon
// clipPIng: 0 - there were intersections or the line was empty; 1 - no
// intersections 2 - there were intersections, and the first and last segments
// should be rejoined.

pub struct Line {
  c0: u8,    // code for previous point
  clean: u8, // no intersections
  radius: f64,
  rc: f64,
  not_hemisphere: bool,
  // point0: (Option<Point>, Option<u8>), // previous point with message.
  point0: Option<Point>, // previous point
  small_radius: bool,
  stream: Rc<RefCell<Box<dyn TransformStream>>>,
  v0: bool,  // visibility of previous point
  v00: bool, // visibility of first point
  visible: Rc<PointsVisibleFn>,
}

impl Line {
  pub fn new(visible: Rc<PointsVisibleFn>, radius: f64) -> StreamProcessor {
    return Box::new(move |stream_ptr: Rc<RefCell<Box<dyn TransformStream>>>| {
      let stream = stream_ptr.clone();
      // TODO small_radius, rc  is a shadow variables!!!
      let rc = radius.cos();
      let small_radius = rc > 0f64;
      return Rc::new(RefCell::new(Box::new(Line {
        c0: 0,
        clean: 0,
        not_hemisphere: rc.abs() > f64::EPSILON,
        point0: None,
        rc,
        radius,
        small_radius,
        v0: false,
        v00: false,
        stream,
        visible: visible.clone(),
      })));
    });
  }

  /// Generates a 4-bit vector representing the location of a point relative to
  /// the small circle's bounding box.
  fn code(&self, lambda: f64, phi: f64) -> u8 {
    let r = match self.small_radius {
      true => self.radius,
      false => f64::consts::PI - self.radius,
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

impl TransformStream for Line {
  fn line_start(&mut self) {
    self.v00 = false;
    self.v0 = false;
    self.clean = 1;
  }

  fn point(&mut self, lambda: f64, phi: f64, m: Option<u8>) {
    let mut point1 = Point { x: lambda, y: phi };

    // let point2: (Option::<Point>, <Option<u8>>);
    let mut point2: (Option<Point>, Option<u8>);
    let v = (self.visible)(lambda, phi, None);

    let c = match self.small_radius {
      true => match v {
        true => 0u8,
        false => self.code(lambda, phi),
      },
      false => match v {
        true => {
          let inc = match lambda < 0f64 {
            true => f64::consts::PI,
            false => -f64::consts::PI,
          };
          self.code(lambda + inc, phi)
        }
        false => 0u8,
      },
    };

    if self.point0.is_none() {
      self.v00 = v;
      self.v0 = v;
      if v {
        let mut stream = self.stream.borrow_mut();
        stream.line_start();
      }
    }

    if v != self.v0 {
      let point2 = intersect(
        self.point0.clone().unwrap(),
        point1.clone(),
        self.radius.cos(),
        false,
      );
      match point2 {
        IntersectReturn::None => {
          point1.x = 1f64;
        }
        IntersectReturn::One(p) => {
          if point_equal(self.point0.clone().unwrap(), p.clone()) || point_equal(point1.clone(), p) {
            point1.x = 1f64;
          }
        }
        IntersectReturn::Two(_t) => {
          // There is a subtle bug in the javascript here two points is handles
          // as if the second does not exits.
          // For now just cause a panic here to see how many times it occurs.
          panic!("requested One or None found Two as !!");
        }
      }
    }

    let mut stream = self.stream.borrow_mut();
    if v != self.v0 {
      let next: Option<Point>;
      self.clean = 0;
      if v {
        // outside going in
        stream.line_start();
        match intersect(point1, self.point0.clone().unwrap(), self.rc, false) {
          IntersectReturn::None => {
            // TODO Should I do a stream Point here??
            next = None;
          }
          IntersectReturn::One(p) => {
            stream.point(p.x, p.y, None);
            next = Some(p);
          }
          IntersectReturn::Two([p, _]) => {
            stream.point(p.x, p.y, None);
            // p0_next = p;
            panic!("silently dropping second point");
          }
        }
      } else {
        // inside going out
        let point2 = intersect(self.point0.clone().unwrap(), point1, self.rc, false);
        match point2 {
          IntersectReturn::None => {
            // TODO should I stream a null point here?
            // stream.line_end(); ???
            panic!("Must deal with no intersect.");
          }
          IntersectReturn::One(p) => {
            stream.point(p.x, p.y, Some(2));
            stream.line_end();
            next = Some(p);
          }
          IntersectReturn::Two([p, _]) => {
            stream.point(p.x, p.y, Some(2));
            stream.line_end();
            // next = p;
            panic!("silently dropping second point");
          }
        }
      }
      self.point0 = next;
    } else if self.not_hemisphere && self.point0.is_none() && self.small_radius ^ v {
      // If the codes for two points are different, or are both zero,
      // and there this segment intersects with the small circle.
      if (c & self.c0) != 0 {
        // let t = intersect(point1.0.unwrap(), self.point0.0.unwrap(), self.rc, true);
        // match t {
        //   Return::None => {}
        //   Return::One(_) => {
        //     panic!("requetsed two received one");
        //   }
        //   Return::Two(t) => {
        //     self.clean = 0;
        //     if self.small_radius {
        //       stream.line_start();
        //       stream.point(t[0].x, t[0].y, None);
        //       stream.point(t[1].x, t[1].y, None);
        //       stream.line_end();
        //     } else {
        //       stream.point(t[1].x, t[1].y, None);
        //       stream.line_end();
        //       stream.line_start();
        //       stream.point(t[0].x, t[0].y, Some(3u8));
        //     }
        //   }
        // }
      }
    }
    // if v && self.point0.0.is_none() || !point_equal(self.point0.0.unwrap(), point1.0.unwrap()) {
    //   stream.point(point1.0.unwrap().x, point1.0.unwrap().y, None);
    // }
    // self.point0 = point1;
    self.v0 = v;
    self.c0 = c;
  }

  fn line_end(&mut self) {
    if self.v0 {
      let mut stream = self.stream.borrow_mut();
      stream.line_end();
    }
    self.point0 = None;
  }
}
// function clipLine(stream) {
//   var point0, // previous point
//       c0, // code for previous point
//       v0, // visibility of previous point
//       v00, // visibility of first point
//       clean; // no intersections
//   return {
//     lineStart: f64unction() {
//       v00 = v0 = false;
//       clean = 1;
//     },
//     point: f64unction(lambda, phi) {
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
//     lineEnd: f64unction() {
//       if (v0) stream.lineEnd();
//       point0 = null;
//     },
//     // Rejoin first and last segments if there were intersections and the first
//     // and last points were visible.
//     clean: f64unction() {
//       return clean | ((v00 && v0) << 1);
//     }
//   };
// }
