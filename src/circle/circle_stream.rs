use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical_r;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::Transform;

use super::circle_generator::CircleGenerator;
use super::circle_radius::circle_radius;

/// Generates a circle centered at [0°, 0°], with a given radius and precision.
pub fn circle_stream<
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
>(
    circle: &mut impl Stream<T, C = Coordinate<T>>,
    radius: T,
    delta: T,
    direction: T,
    p0: Option<Coordinate<T>>,
    p1: Option<Coordinate<T>>,
) {
    if delta.is_zero() {
        return;
    }
    let cos_radius = radius.cos();
    let sin_radius = radius.sin();
    let step = direction * delta;
    let mut t0: T;
    let t1: T;
    match (p0, p1) {
        (Some(p0), Some(p1)) => {
            t0 = circle_radius(cos_radius, p0);
            t1 = circle_radius(cos_radius, p1);
            let check = match direction.is_sign_positive() {
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

    let mut point: Coordinate<T>;
    let mut t = t0;
    let mut cond = true;
    while cond {
        point = spherical_r(&[cos_radius, -sin_radius * t.cos(), -sin_radius * t.sin()]);
        circle.point(&point, None);

        t = t - step;
        cond = match direction.is_sign_positive() {
            true => t > t1,
            false => t < t1,
        };
    }
}
