use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::cartesian::spherical_radians;
use crate::stream::Stream;

use super::calc_radius::calc_radius;

/// Generates a circle centered at [0°, 0°], with a given radius and precision.
///
/// # Panics
/// `unwrap()` is used here but a panic will never happen as 2 will always be converted into T.
pub fn stream_fn<EP, STREAM, T>(
    stream: &mut STREAM,
    radius: T,
    delta: T,
    direction: T,
    t0_in: Option<Coord<T>>,
    t1_in: Option<Coord<T>>,
) where
    STREAM: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    if delta.is_zero() {
        return;
    }
    let (sin_radius, cos_radius) = radius.sin_cos();
    let step = direction * delta;
    let mut t0: T;
    let t1: T;

    if let (Some(p0), Some(p1)) = (t0_in, t1_in) {
        t0 = calc_radius(cos_radius, &p0);
        t1 = calc_radius(cos_radius, &p1);
        let check = if direction > T::zero() {
            t0 < t1
        } else {
            t0 > t1
        };

        if check {
            t0 = t0 + direction * T::TAU();
        }
    } else {
        t0 = radius + direction * T::TAU();
        t1 = radius - step / T::from(2).unwrap();
    }

    let mut t = t0;
    let mut cond = true;
    while cond {
        let (st, ct) = t.sin_cos();
        let point = spherical_radians(&[
            cos_radius,
            -sin_radius * ct,
            -sin_radius * st,
        ]);
        stream.point(&point, None);

        t = t - step;
        cond = if direction > T::zero() {
            t > t1
        } else {
            t < t1
        };
    }
}
