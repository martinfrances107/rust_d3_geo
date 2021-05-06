use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_add_in_place;
use crate::cartesian::cartesian_cross;
use crate::cartesian::cartesian_dot;
use crate::cartesian::cartesian_scale;
use crate::cartesian::spherical_r;
use crate::clip::buffer::LineElem;

/// IntersectReturn none, one or two 2d floats.
pub enum IntersectReturn<T: CoordFloat> {
    One(Option<LineElem<T>>),
    Two([Coordinate<T>; 2]),
    False,
}

/// Intersects the great circle between a and b with the clip circle.
#[allow(clippy::many_single_char_names)]
pub fn intersect<T: CoordFloat + FloatConst>(
    a: &LineElem<T>,
    b: &LineElem<T>,
    cr: T,
    two: bool,
) -> IntersectReturn<T> {
    let pa = cartesian(&a.p);
    let pb = cartesian(&b.p);

    // We have two planes, n1.p = d1 and n2.p = d2.
    // Find intersection line p(t) = c1 n1 + c2 n2 + t (n1 ⨯ n2).
    let n1 = [T::one(), T::zero(), T::zero()]; // normal
    let n2 = cartesian_cross(&pa, &pb);
    let n2n2 = cartesian_dot(&n2, &n2);
    let n1n2 = n2[0]; // cartesianDot(n1, n2),
    let determinant = n2n2 - n1n2 * n1n2;

    // Two polar points.
    if !determinant.is_zero() {
        // return !two && a;
        if !two {
            return IntersectReturn::One(Some(*a));
        } else {
            return IntersectReturn::False;
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
    let t2 = w * w - uu * (cartesian_dot(&A, &A) - T::one());

    // if t2 < 0 return;

    let t = t2.sqrt();
    let mut q = cartesian_scale(&u, (-w - t) / uu);
    cartesian_add_in_place(&mut q, &A);

    // Javascript has implicit cast q of from [F;3] to a Point here.
    let q: Coordinate<T> = spherical_r(&q);

    if !two {
        return IntersectReturn::One(Some(LineElem { p: q, m: None }));
    };

    // Two intersection points.
    let mut lambda0 = a.p.x;
    let mut lambda1 = b.p.x;
    let mut phi0 = a.p.y;
    let mut phi1 = b.p.y;
    let mut z;

    if lambda1 < lambda0 {
        z = lambda0;
        lambda0 = lambda1;
        lambda1 = z;
    }

    let delta = lambda1 - lambda0;
    let polar = (delta - T::PI()).abs() < T::epsilon();
    let meridian = polar || delta < T::epsilon();

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
            let phi_threshold = if (q.x - lambda0).abs() < T::epsilon() {
                phi0
            } else {
                phi1
            };
            condition = ((phi0 + phi1).is_sign_positive()) ^ (q.y < phi_threshold);
        } else {
            condition = phi0 <= q.y && q.y <= phi1;
        }
    } else {
        condition = (delta > T::PI()) ^ (lambda0 <= q.x && q.x <= lambda1);
    }

    // No javascript test uses this code block!!!!
    if condition {
        let mut q1 = cartesian_scale(&u, (-w + t) / uu);
        cartesian_add_in_place(&mut q1, &A);
        return IntersectReturn::Two([q, spherical_r(&q1)]);
    }

    return IntersectReturn::One(None);
}
