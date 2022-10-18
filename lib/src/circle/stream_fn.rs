use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::cartesian::spherical_radians;
use crate::stream::Stream;

use super::calc_radius::calc_radius;

/// Generates a circle centered at [0°, 0°], with a given radius and precision.
pub fn stream_fn<EP, STREAM, T>(
    stream: &mut STREAM,
    radius: T,
    delta: T,
    direction: T,
    t0_in: Option<Coordinate<T>>,
    t1_in: Option<Coordinate<T>>,
) where
    T: CoordFloat + FloatConst,
    STREAM: Stream<EP = EP, T = T>,
{
    if delta.is_zero() {
        return;
    }
    let (sin_radius, cos_radius) = radius.sin_cos();
    let step = direction * delta;
    let mut t0: T;
    let t1: T;
    match (t0_in, t1_in) {
        (Some(p0), Some(p1)) => {
            t0 = calc_radius(cos_radius, &p0);
            t1 = calc_radius(cos_radius, &p1);
            let check = match direction > T::zero() {
                true => t0 < t1,
                false => t0 > t1,
            };
            if check {
                t0 = t0 + direction * T::TAU();
            }
        }
        (_, _) => {
            t0 = radius + direction * T::TAU();
            t1 = radius - step / T::from(2).unwrap();
        }
    }

    let mut t = t0;
    let mut cond = true;
    while cond {
        let point = spherical_radians(&[cos_radius, -sin_radius * t.cos(), -sin_radius * t.sin()]);
        stream.point(&point, None);

        t = t - step;
        cond = match direction > T::zero() {
            true => t > t1,
            false => t < t1,
        };
    }
}
