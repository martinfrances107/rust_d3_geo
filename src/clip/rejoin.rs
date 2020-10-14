use delaunator::Point;

// use crate::stream::GeoStream;
use crate::point_equal::point_equal;
use crate::transform_stream::TransformStream;

use super::CompareIntersectionFn;
use super::InterpolateFn;

// import pointEqual from "../pointEqual.js";

type MeshPoint = [f64; 3];

struct Intersection
{
  x: MeshPoint,
  z: Option<Vec<MeshPoint>>,
  o: Option<Box<Intersection>>, // another intersection,
  e: bool,                         // is any entry?
  v: bool,                         // visited
  n: Option<MeshPoint>,         // next
  p: Option<MeshPoint>,         // previous
}

impl Intersection
{
  fn new(
    point: MeshPoint,
    points: Option<Vec<MeshPoint>>,
    other: Option<Box<Intersection>>,
    entry: bool,
  ) -> Self {
    return Self {
      x: point,
      z: points,
      o: other,
      e: entry,
      v: false,
      n: None,
      p: None,
    };
  }
}

//
// function Intersection(point, points, other, entry) {
//   this.x = point;
//   this.z = points;
//   this.o = other; // another intersection
//   this.e = entry; // is an entry?
//   this.v = false; // visited
//   this.n = this.p = null; // next & previous
// }

/// A generalized polygon clipping algorithm: given a polygon that has been cut
/// into its visible line segments, and rejoins the segments by interpolating
/// along the clip edge.
pub fn rejoin(
  segments: Vec<Vec<MeshPoint>>,
  compare_intersection: CompareIntersectionFn,
  start_inside: bool,
  interpolate: InterpolateFn,
  mut stream: Box<dyn TransformStream>,
) {
  let subject = Vec::<Intersection>::new();
  let clip = Vec::<Intersection>::new();
  // let i,
  // let n: usize;

  for segment in segments.iter() {
    let n = segment.len() - 1;
    if n <= 0 {
      return;
    };
    let mut p0 = segment[0];
    let mut p1 = segment[n];
    //  let mut x: Intersection<F>;

    if point_equal(Point{x:p0[0], y:p0[1]}, Point{x:p1[0], y:p1[1]}) {
      if p0[2] != 0f64 && p1[2] != 0f64 {
        stream.line_start();
        // let i: usize;
        // for (i = 0; i < n; ++i) stream.point((p0 = segment[i])[0], p0[1]);
        for i in 0..n {
          p0 = segment[i];
          stream.point(p0[0], p0[1], None);
        }
        stream.line_end();
        return;
      }
      // handle degenerate cases by moving the point
      // p1[0] += 2F * f64::EPSILON;
      p1[0] = p1[0] + 2f64 * f64::EPSILON;
    }

    // let mut x = Intersection::new(p0, Some(segment.to_vec()), None, true);
    // subject.push(x);
    // x.o = Some(Box::new(Intersection::new(
    //   p0,
    //   None,
    //   Some(Box::new(x)),
    //   false,
    // )));
    // clip.push(*x.o.unwrap());
    // x = Intersection::new(p1, Some(segment.to_vec()), None, false);
    // subject.push(x);
    // x.o = Some(Box::new(Intersection::new(
    //   p1,
    //   None,
    //   Some(Box::new(x)),
    //   true,
    // )));
    // clip.push(*x.o.unwrap());
  }
}

// // A generalized polygon clipping algorithm: given a polygon that has been cut
// // into its visible line segments, and rejoins the segments by interpolating
// // along the clip edge.
// export default function(segments, compareIntersection, startInside, interpolate, stream) {
//   var subject = [],
//       clip = [],
//       i,
//       n;

//   segments.forEach(function(segment) {
//     if ((n = segment.length - 1) <= 0) return;
//     var n, p0 = segment[0], p1 = segment[n], x;

//     if (pointEqual(p0, p1)) {
//       if (!p0[2] && !p1[2]) {
//         stream.lineStart();
//         for (i = 0; i < n; ++i) stream.point((p0 = segment[i])[0], p0[1]);
//         stream.lineEnd();
//         return;
//       }
//       // handle degenerate cases by moving the point
//       p1[0] += 2 * epsilon;
//     }

//     subject.push(x = new Intersection(p0, segment, null, true));
//     clip.push(x.o = new Intersection(p0, null, x, false));
//     subject.push(x = new Intersection(p1, segment, null, false));
//     clip.push(x.o = new Intersection(p1, null, x, true));
//   });

//   if (!subject.length) return;

//   clip.sort(compareIntersection);
//   link(subject);
//   link(clip);

//   for (i = 0, n = clip.length; i < n; ++i) {
//     clip[i].e = startInside = !startInside;
//   }

//   var start = subject[0],
//       points,
//       point;

//   while (1) {
//     // Find first unvisited intersection.
//     var current = start,
//         isSubject = true;
//     while (current.v) if ((current = current.n) === start) return;
//     points = current.z;
//     stream.lineStart();
//     do {
//       current.v = current.o.v = true;
//       if (current.e) {
//         if (isSubject) {
//           for (i = 0, n = points.length; i < n; ++i) stream.point((point = points[i])[0], point[1]);
//         } else {
//           interpolate(current.x, current.n.x, 1, stream);
//         }
//         current = current.n;
//       } else {
//         if (isSubject) {
//           points = current.p.z;
//           for (i = points.length - 1; i >= 0; --i) stream.point((point = points[i])[0], point[1]);
//         } else {
//           interpolate(current.x, current.p.x, -1, stream);
//         }
//         current = current.p;
//       }
//       current = current.o;
//       points = current.z;
//       isSubject = !isSubject;
//     } while (!current.v);
//     stream.lineEnd();
//   }
// }

struct LinkNP<'a, T> {
  value: T,
  n: Option<&'a LinkNP<'a, T>>,
  p: Option<&'a LinkNP<'a, T>>,
}

fn link(array: Vec<MeshPoint>)
{
  if array.is_empty() {
    return;
  };
  let n = array.len();

  let i: usize = 0usize;
  let mut a = LinkNP {
    value: array[0],
    n: None,
    p: None,
  };
  let mut b: LinkNP<MeshPoint>;
  for i in 1..n {
    b = LinkNP {
      value: array[i],
      n: None,
      p: None,
    };
    // a.n = Some(&b);
    // b.p = Some(&a);
    a = b;
  }
  b = LinkNP {
    value: array[i],
    n: None,
    p: None,
  };
  // a.n = Some(&b);
  // b.p = Some(&a);
}

// function link(array) {
//   if (!(n = array.length)) return;
//   var n,
//       i = 0,
//       a = array[0],
//       b;
//   while (++i < n) {
//     a.n = b = array[i];
//     b.p = a;
//     a = b;
//   }
//   a.n = b = array[0];
//   b.p = a;
// }
