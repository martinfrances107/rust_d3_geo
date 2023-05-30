use core::fmt::Display;

use geo::CoordFloat;
use geo::Point;
use geo_types::Coord;

/// True if two points are identical within the specified difference.
pub fn in_delta<T: CoordFloat + Display>(actual: T, expected: T, delta: T) -> bool {
    println!("in_delta() expected {expected:?} actual {actual:?} delta {delta:?}");

    let is_ok = if actual.is_nan() && expected.is_nan() {
        true
    } else {
        (actual - expected).abs() <= delta
    };

    if !is_ok {
        println!(
            "in_delta() FAIL: expected delta {}  - actual delta {}",
            delta,
            actual - expected
        );
    }
    is_ok
}

/// Returns true if two points are considered equal, within the specified differnce.
///
/// Debug and test helper function.
pub fn point<T: CoordFloat + Display>(actual: Point<T>, expected: Point<T>, delta: T) -> bool {
    println!("in_delta_point: expected(Point) {expected:?} actual {actual:?} delta {delta:?}");
    let x = in_delta(actual.x(), expected.x(), delta);
    println!("x: {x}");
    let y = in_delta(actual.y(), expected.y(), delta);
    println!("y: {y}");
    x && y
}

/// Returns true if two points are considered equal, within the specified differnce.
///
/// Debug and test helper function.
pub fn coordinate<T: CoordFloat + Display>(
    actual: &Coord<T>,
    expected: &Coord<T>,
    delta: T,
) -> bool {
    println!("in_delta_point: expected(Point) {expected:?} actual {actual:?} delta {delta:?}");
    in_delta(actual.x, expected.x, delta) && in_delta(actual.y, expected.y, delta)
}
