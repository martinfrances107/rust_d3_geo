use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::stream::GeoStream;
use crate::point_equal::point_equal;

use super::InterpolateFn;
use super::CompareIntersectionFn;

// import pointEqual from "../pointEqual.js";

type MeshPoint<F> = (F,F,bool);

struct Intersection<F>
where F: Float {
  x: MeshPoint<F>,
  z: Option<Vec<MeshPoint<F>>>,
  o: Option<MeshPoint<F>>,// another intersection,
  e: bool, // is any entry?
  v: bool, // visited
  n: Option<MeshPoint<F>>, // next
  p: Option<MeshPoint<F>>, // previous
}

impl<F> Intersection<F>
where F: Float {
  fn new(point: MeshPoint<F>, points: Option<Vec<MeshPoint<F>>>, other: Option<MeshPoint<F>>, entry: bool) -> Self {
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
pub fn rejoin<F>(segments: Vec<Vec<(F,F,bool)>>, compare_intersection: CompareIntersectionFn<F>, start_inside: bool, interpolate: InterpolateFn<F>, stream: Box<dyn GeoStream<F>>)
where F: Float + FromPrimitive {
  let subject = Vec::<Intersection<F>>::new();
  let clip: Vec<[F;2]>;
  // let i,
  let n:usize;

  for segment in segments.iter() {
    n = segment.len() - 1;
    if n <= 0 {return};
    let p0 = segment[0];
    let p1 = segment[n];
    let x;

    if point_equal(p0, p1) {
      if !p0.2 && !p1.2 {
        stream.line_start();
        let i;
        // for (i = 0; i < n; ++i) stream.point((p0 = segment[i])[0], p0[1]);
        for i in  0..n {
          p0 = segment[i];
          stream.point(p0.0, p0.1);
        }
        stream.line_end();
        return;
      }
      // handle degenerate cases by moving the point
      // p1[0] += 2F * F::epsilon();
      p1.0 = p1.0 + F::from(2u8: u8).unwrap() * F::epsilon();
    }

    let x = Intersection::new(p0, Some(segment), None, true);
    subject.push(x);
    x.o = Some(Intersection::new(p0, None, Some(x), false));
    clip.push(x.o);
    x = Intersection::new(p1, Some(segment), None, false);
    subject.push(x);
    x.o = Intersection::new(p1, None, Some(x), true);
    clip.push(x.o);

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


fn link<F>(array: Vec<MeshPoint<F>>)
where F: Float {
  if array.is_empty() { return };
  let n = array.len();

  let i:usize = 0usize;
  let a = array[0];
  let b;
  while  (i = i + 1) < n {
    a.n = b = array[i];
    b.p = a;
    a = b;
  }
  a.n = b = array[0];
  b.p = a;
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
