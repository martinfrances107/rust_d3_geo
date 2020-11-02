use crate::cartesian::cartesian;
use crate::cartesian::cartesian_add_in_place;
use crate::cartesian::cartesian_cross;
use crate::cartesian::cartesian_dot;
use crate::cartesian::cartesian_scale;
use crate::cartesian::spherical;
use delaunator::Point;
use std::f64;

/// IntersectReturn none, one or two 2d floats.
pub enum IntersectReturn {
    One(Point),
    Two([Point; 2]),
    None,
}

/// Intersects the great circle between a and b with the clip circle.
#[allow(clippy::many_single_char_names)]
pub fn intersect(a: Point, b: Point, cr: f64, two: bool) -> IntersectReturn {
    let pa = cartesian(&a);
    let pb = cartesian(&b);

    // We have two planes, n1.p = d1 and n2.p = d2.
    // Find intersection line p(t) = c1 n1 + c2 n2 + t (n1 ⨯ n2).
    let n1 = [1f64, 0f64, 0f64]; // normal
    let n2 = cartesian_cross(&pa, &pb);
    let n2n2 = cartesian_dot(&n2, &n2);
    let n1n2 = n2[0]; // cartesianDot(n1, n2),
    let determinant = n2n2 - n1n2 * n1n2;

    // Two polar points.
    if determinant != 0f64 {
        // return !two && a;
        if !two {
            return IntersectReturn::One(a);
        } else {
            return IntersectReturn::None;
        }
    };

    let c1 = cr * n2n2 / determinant;
    let c2 = -cr * n1n2 / determinant;
    let n1xn2 = cartesian_cross(&n1, &n2);
    #[allow(non_snake_case)]
    let mut A = cartesian_scale(&n1, c1);
    #[allow(non_snake_case)]
    let B = cartesian_scale(&n2, c2);
    cartesian_add_in_place(&mut A, &B);

    // Solve |p(t)|^2 = 1.
    let u = n1xn2;
    let w = cartesian_dot(&A, &u);
    let uu = cartesian_dot(&u, &u);
    let t2 = w * w - uu * (cartesian_dot(&A, &A) - 1f64);

    // if t2 < 0 return;

    let t = t2.sqrt();
    let mut q = cartesian_scale(&u, (-w - t) / uu);
    cartesian_add_in_place(&mut q, &A);

    // Javascript has implicit cast q of from [F;3] to a Point here.
    let q: Point = spherical(&q);

    if !two {
        return IntersectReturn::One(q);
    };

    // Two intersection points.
    let mut lambda0 = a.x;
    let mut lambda1 = b.x;
    let mut phi0 = a.y;
    let mut phi1 = b.y;
    let mut z;

    if lambda1 < lambda0 {
        z = lambda0;
        lambda0 = lambda1;
        lambda1 = z;
    }

    let delta = lambda1 - lambda0;
    let polar = (delta - f64::consts::PI).abs() < f64::EPSILON;
    let meridian = polar || delta < f64::EPSILON;

    if !polar && phi1 < phi0 {
        z = phi0;
        phi0 = phi1;
        phi1 = z
    };

    // if (meridian
    //     ? polar
    //       ? phi0 + phi1 > 0 ^ q[1] < (abs(q[0] - lambda0) < epsilon ? phi0 : phi1)
    //       : phi0 <= q[1] && q[1] <= phi1
    //     : delta > PI ^ (lambda0 <= q[0] && q[0] <= lambda1)) {
    //   var q1 = cartesianScale(u, (-w + t) / uu);
    //   cartesianAddInPlace(q1, A);
    //   return [q, spherical(q1)];
    // }

    // Check that the first point is between a and b.
    let condition: bool;
    if meridian {
        if polar {
            let phi_threshold = if (q.x - lambda0).abs() < f64::EPSILON {
                phi0
            } else {
                phi1
            };
            condition = ((phi0 + phi1) > 0f64) ^ (q.y < phi_threshold);
        } else {
            condition = phi0 <= q.y && q.y <= phi1;
        }
    } else {
        condition = (delta > f64::consts::PI) ^ (lambda0 <= q.x && q.x <= lambda1);
    }

    // Not javascript test exits to test this code block!!!!
    // TODO must fix when I understand the return type [q, spherical(&q1)]!!!
    if condition {
        let mut q1 = cartesian_scale(&u, (-w + t) / uu);
        cartesian_add_in_place(&mut q1, &A);
        return IntersectReturn::Two([q, spherical(&q1)]);
    }

    return IntersectReturn::None;
}

// // Intersects the great circle between a and b with the clip circle.
// function intersect(a, b, two) {
//     var pa = cartesian(a),
//         pb = cartesian(b);

//     // We have two planes, n1.p = d1 and n2.p = d2.
//     // Find intersection line p(t) = c1 n1 + c2 n2 + t (n1 ⨯ n2).
//     var n1 = [1, 0, 0], // normal
//         n2 = cartesianCross(pa, pb),
//         n2n2 = cartesianDot(n2, n2),
//         n1n2 = n2[0], // cartesianDot(n1, n2),
//         determinant = n2n2 - n1n2 * n1n2;

//     // Two polar points.
//     if (!determinant) return !two && a;

//     var c1 =  cr * n2n2 / determinant,
//         c2 = -cr * n1n2 / determinant,
//         n1xn2 = cartesianCross(n1, n2),
//         A = cartesianScale(n1, c1),
//         B = cartesianScale(n2, c2);
//     cartesianAddInPlace(A, B);

//     // Solve |p(t)|^2 = 1.
//     var u = n1xn2,
//         w = cartesianDot(A, u),
//         uu = cartesianDot(u, u),
//         t2 = w * w - uu * (cartesianDot(A, A) - 1);

//     if (t2 < 0) return;

//     var t = sqrt(t2),
//         q = cartesianScale(u, (-w - t) / uu);
//     cartesianAddInPlace(q, A);
//     q = spherical(q);

//     if (!two) return q;

//     // Two intersection points.
//     var lambda0 = a[0],
//         lambda1 = b[0],
//         phi0 = a[1],
//         phi1 = b[1],
//         z;

//     if (lambda1 < lambda0) z = lambda0, lambda0 = lambda1, lambda1 = z;

//     var delta = lambda1 - lambda0,
//         polar = abs(delta - PI) < epsilon,
//         meridian = polar || delta < epsilon;

//     if (!polar && phi1 < phi0) z = phi0, phi0 = phi1, phi1 = z;

//     // Check that the first point is between a and b.
//     if (meridian
//         ? polar
//           ? phi0 + phi1 > 0 ^ q[1] < (abs(q[0] - lambda0) < epsilon ? phi0 : phi1)
//           : phi0 <= q[1] && q[1] <= phi1
//         : delta > PI ^ (lambda0 <= q[0] && q[0] <= lambda1)) {
//       var q1 = cartesianScale(u, (-w + t) / uu);
//       cartesianAddInPlace(q1, A);
//       return [q, spherical(q1)];
//     }
//   }
