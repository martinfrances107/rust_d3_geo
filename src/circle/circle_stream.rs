use std::cell::RefCell;
use std::f64;
use std::rc::Rc;

use delaunator::Point;

use crate::cartesian::cartesian;
use crate::cartesian::cartesian_normalize_in_place;
use crate::cartesian::spherical;
use crate::math::TAU;
use crate::transform_stream::TransformStream;
use crate::Transform;

use super::circle_radius::circle_radius;

/// Generates a circle centered at [0°, 0°], with a given radius and precision.
pub fn circle_stream(
    stream: Rc<RefCell<Box<dyn TransformStream>>>,
    radius: f64,
    delta: f64,
    direction: f64,
    p0: Option<Point>,
    p1: Option<Point>,
) {
    if delta == 0f64 {
        return;
    };
    let cos_radius = radius.cos();
    let sin_radius = radius.sin();
    let step = direction * delta;
    let mut t0: f64;
    let t1: f64;
    match (p0, p1) {
        (Some(p0), Some(p1)) => {
            t0 = circle_radius(cos_radius, p0);
            t1 = circle_radius(cos_radius, p1);
            let check = match direction > 0f64 {
                true => t0 < t1,
                false => t0 > t1,
            };
            if check {
                t0 = t0 + direction * TAU;
            }
        }
        (_, _) => {
            t0 = radius + direction * TAU;
            t1 = radius - step / 2f64;
        }
    }

    let mut point: Point;
    let mut t = t0;
    let mut cond = true;
    let mut stream = stream.borrow_mut();
    while cond {
        point = spherical(&[cos_radius, -sin_radius * t.cos(), -sin_radius * t.sin()]);
        stream.point(point.x, point.y, None);

        t = t - step;
        cond = match direction > 0f64 {
            true => t > t1,
            false => t < t1,
        };
    }
}
