use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use crate::cartesian::cartesian;
use crate::cartesian::normalize_in_place;
use crate::cartesian::spherical_r;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::Transform;

use super::calc_radius::calc_radius;
use super::generator::Generator;

/// Generates a circle centered at [0°, 0°], with a given radius and precision.
pub fn generate<STREAM, T>(
    circle: Rc<RefCell<STREAM>>,
    radius: T,
    delta: T,
    direction: T,
    p0: Option<Coordinate<T>>,
    p1: Option<Coordinate<T>>,
) where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    STREAM: Stream<SC = Coordinate<T>>,
{
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
            t0 = calc_radius(cos_radius, p0);
            t1 = calc_radius(cos_radius, p1);
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
        circle.borrow_mut().point(&point, None);

        t = t - step;
        cond = match direction.is_sign_positive() {
            true => t > t1,
            false => t < t1,
        };
    }
}
