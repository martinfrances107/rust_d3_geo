use geo::{CoordFloat, Point};
use std::fmt::Debug;
use std::fmt::Display;

pub fn in_delta<T: CoordFloat + Debug + Display>(actual: T, expected: T, delta: T) -> bool {
    println!(
        "expected {:?} actual {:?} delta {:?}",
        expected, actual, delta
    );
    let is_ok = (actual - expected).abs() <= delta;
    if !is_ok {
        println!(
            "FAIL: expected delta {}  - actual delta {}",
            delta,
            actual - expected
        );
    }
    return is_ok;
}

pub fn in_delta_point<T: CoordFloat + Debug + Display>(
    actual: Point<T>,
    expected: Point<T>,
    delta: T,
) -> bool {
    println!(
        "expected(Point) {:?} actual {:?} delta {:?}",
        expected, actual, delta
    );
    return in_delta(actual.x(), expected.x(), delta) && in_delta(actual.y(), expected.y(), delta);
}
