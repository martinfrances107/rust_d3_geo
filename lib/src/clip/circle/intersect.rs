use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::cartesian::add_in_place;
use crate::cartesian::cartesian;
use crate::cartesian::cross;
use crate::cartesian::dot;
use crate::cartesian::scale;
use crate::cartesian::spherical_radians;
use crate::clip::line_elem::LineElem;
use crate::math::EPSILON;

/// `IntersectReturn` none, one or two 2d floats.
#[derive(Debug)]
pub(super) enum Return<T: CoordFloat> {
    /// One Point.
    One(Option<LineElem<T>>),
    /// Two polar points
    Two([Coord<T>; 2]),
    /// TODO can I remove,
    False,
    /// No Intersection.
    None,
}

/// Intersects the great circle between a and b with the clip circle.
///
/// # Panics
/// `unwrap()` is used here but a panic will never happen as EPSILON will always be converted into T.
#[allow(clippy::many_single_char_names)]
#[allow(clippy::similar_names)]
#[allow(non_snake_case)]
pub(super) fn intersect<T: CoordFloat + FloatConst>(
    a: &LineElem<T>,
    b: &LineElem<T>,
    cr: T,
    two: bool,
) -> Return<T> {
    let pa = cartesian(&a.p);
    let pb = cartesian(&b.p);

    // We have two planes, n1.p = d1 and n2.p = d2.
    // Find intersection line p(t) = c1 n1 + c2 n2 + t (n1 ⨯ n2).
    let n1 = [T::one(), T::zero(), T::zero()]; // normal
    let n2 = cross(&pa, &pb);
    let n2n2 = dot(&n2, &n2);
    let n1n2 = n2[0]; // cartesianDot(n1, n2),
    let determinant = n2n2 - n1n2 * n1n2;

    // Two polar points.
    if determinant.is_zero() {
        return if two {
            Return::False
        } else {
            Return::One(Some(*a))
        };
    };

    let c1 = cr * n2n2 / determinant;
    let c2 = -cr * n1n2 / determinant;
    let n1xn2 = cross(&n1, &n2);

    let mut A = scale(&n1, c1);
    let B = scale(&n2, c2);
    add_in_place(&mut A, &B);

    // Solve |p(t)|^2 = 1.
    let u = n1xn2;
    let w = dot(&A, &u);
    let uu = dot(&u, &u);
    let t2 = w * w - uu * (dot(&A, &A) - T::one());

    if t2 < T::zero() {
        return Return::None;
    }

    let t = t2.sqrt();
    let mut q = scale(&u, (-w - t) / uu);
    add_in_place(&mut q, &A);
    // Javascript has implicit cast q of from [F;3] to a Point here.
    let q: Coord<T> = spherical_radians(&q);

    if !two {
        return Return::One(Some(LineElem { p: q, m: None }));
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
    let epsilon = T::from(EPSILON).unwrap();
    let polar = (delta - T::PI()).abs() < epsilon;
    let meridian = polar || delta < epsilon;

    if !polar && phi1 < phi0 {
        z = phi0;
        phi0 = phi1;
        phi1 = z;
    };

    // Check that the first point is between a and b.
    let condition: bool;
    if meridian {
        if polar {
            let phi_threshold = if (q.x - lambda0).abs() < epsilon {
                phi0
            } else {
                phi1
            };
            condition =
                ((phi0 + phi1).is_sign_positive()) ^ (q.y < phi_threshold);
        } else {
            condition = phi0 <= q.y && q.y <= phi1;
        }
    } else {
        condition = (delta > T::PI()) ^ (lambda0 <= q.x && q.x <= lambda1);
    }

    // The only test to implement this is snapshot: gnomic
    // so as yet this is untested.
    if condition {
        let mut q1 = scale(&u, (-w + t) / uu);
        add_in_place(&mut q1, &A);
        return Return::Two([q, spherical_radians(&q1)]);
    }

    Return::One(None)
}
