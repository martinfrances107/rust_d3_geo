use delaunator::Point;

pub fn in_delta(actual: f64, expected: f64, delta: f64) -> bool {
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

pub fn in_delta_point(actual: Point, expected: Point, delta: f64) -> bool {
    println!(
        "expected(Point) {:?} actual {:?} delta {:?}",
        expected, actual, delta
    );
    return in_delta(actual.x, expected.x, delta) && in_delta(actual.y, expected.y, delta);
}
