use delaunator::Point;

pub fn in_delta(actual: f64, expected: f64, delta: f64) -> bool {
    println!(
        "expected {:?} actual {:?} delta {:?}",
        expected, actual, delta
    );
    return (actual - expected).abs() <= delta;
}

pub fn in_delta_point(actual: Point, expected: Point, delta: f64) -> bool {
    println!(
        "expected {:?} actual {:?} delta {:?}",
        expected, actual, delta
    );
    return in_delta(actual.x, expected.x, delta) && in_delta(actual.y, expected.y, delta);
}
