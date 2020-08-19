use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_add_in_place;
use crate::cartesian::cartesian_cross;
use crate::cartesian::cartesian_dot;
use crate::cartesian::cartesian_scale;
use crate::cartesian::spherical;
use crate::math::epsilon;

/// Return none, one or two 2d floats.
pub enum Return<F>
where
    F: Float,
{
    One([F; 2]),
    Two([[F; 2]; 2]),
    None,
}

/// Intersects the great circle between a and b with the clip circle.
pub fn intersect<F>(a: [F; 2], b: [F; 2], cr: F, two: bool) -> Return<F>
where
    F: Float + FloatConst + FromPrimitive,
{
    let pa = cartesian(&a);
    let pb = cartesian(&b);

    // We have two planes, n1.p = d1 and n2.p = d2.
    // Find intersection line p(t) = c1 n1 + c2 n2 + t (n1 ⨯ n2).
    let n1 = [F::one(), F::zero(), F::zero()]; // normal
    let n2 = cartesian_cross(&pa, &pb);
    let n2n2 = cartesian_dot(&n2, &n2);
    let n1n2 = n2[0]; // cartesianDot(n1, n2),
    let determinant = n2n2 - n1n2 * n1n2;

    // Two polar points.
    if !determinant.is_zero() {
        // return !two && a;
        if !two {
            return Return::One(a);
        } else {
            return Return::None;
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
    let t2 = w * w - uu * (cartesian_dot(&A, &A) - F::one());

    // if t2 < 0 return;

    let t = t2.sqrt();
    let mut q = cartesian_scale(&u, (-w - t) / uu);
    cartesian_add_in_place(&mut q, &A);

    // Javascript has implicit cast q of from [F;3] to a [F;2] here.
    let q: [F; 2] = spherical(&mut q);

    if !two {
        return Return::One(q);
    };

    // Two intersection points.
    let mut lambda0 = a[0];
    let mut lambda1 = b[0];
    let mut phi0 = a[1];
    let mut phi1 = b[1];
    let mut z;

    if lambda1 < lambda0 {
        z = lambda0;
        lambda0 = lambda1;
        lambda1 = z;
    }

    let delta = lambda1 - lambda0;
    let polar = (delta - F::PI()).abs() < epsilon::<F>();
    let meridian = polar || delta < epsilon::<F>();

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
            let phi_threshold = if (q[0] - lambda0).abs() < epsilon() {
                phi0
            } else {
                phi1
            };
            condition = ((phi0 + phi1) > F::zero()) ^ (q[1] < phi_threshold);
        } else {
            condition = phi0 <= q[1] && q[1] <= phi1;
        }
    } else {
        condition = (delta > F::PI()) ^ (lambda0 <= q[0] && q[0] <= lambda1);
    }

    // Not javascript test exits to test this code block!!!!
    // TODO must fix when I understand the return type [q, spherical(&q1)]!!!
    if condition {
        let mut q1 = cartesian_scale(&u, (-w + t) / uu);
        cartesian_add_in_place(&mut q1, &A);
        return Return::Two([q, spherical(&q1)]);
    }

    return Return::None;
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
