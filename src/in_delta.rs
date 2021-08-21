use std::fmt::Display;

use geo::{CoordFloat, Point};

pub fn in_delta<T: CoordFloat + Display>(actual: T, expected: T, delta: T) -> bool {
    println!(
        "in_delta() expected {:?} actual {:?} delta {:?}",
        expected, actual, delta
    );
    let is_ok = (actual - expected).abs() <= delta;
    if !is_ok {
        println!(
            "in_delta() FAIL: expected delta {}  - actual delta {}",
            delta,
            actual - expected
        );
    }
    is_ok
}

pub fn in_delta_point<T: CoordFloat + Display>(
    actual: Point<T>,
    expected: Point<T>,
    delta: T,
) -> bool {
    println!(
        "in_delta_point: expected(Point) {:?} actual {:?} delta {:?}",
        expected, actual, delta
    );
    in_delta(actual.x(), expected.x(), delta) && in_delta(actual.y(), expected.y(), delta)
}
