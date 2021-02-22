use geo::{CoordFloat, Coordinate};
use num_traits::{Float, FloatConst};
use std::fmt::Debug;
use std::fmt::Display;

use crate::in_delta::in_delta;
use crate::Transform;

use super::projection_mutator::ProjectionMutator;

pub fn projection_equal<T: CoordFloat + FloatConst + Debug + Display + 'static>(
    projection: &ProjectionMutator<T>,
    expected_location: &Coordinate<T>,
    expected_point: &Coordinate<T>,
    delta_p: Option<T>,
) -> bool {
    let delta = match delta_p {
        Some(d) => d,
        None => T::from(1e-6f64).unwrap(),
    };
    println!("project_equal");
    println!(
        "expected [{:?}, {:?}], [{:?}, {:?}]",
        expected_location.x, expected_location.y, expected_point.x, expected_point.y,
    );
    let actual_location = projection.invert(expected_point);
    let actual_point = projection.transform(expected_location);
    println!(
        "actual [{:?}, {:?}], [{:?}, {:?}]",
        actual_location.x, actual_location.y, actual_point.x, actual_point.y,
    );
    return planar_equal(actual_point, expected_point, delta)
        && spherical_equal(actual_location, expected_location, delta);
}

fn planar_equal<T: CoordFloat + Debug + Display>(
    actual: Coordinate<T>,
    expected: &Coordinate<T>,
    delta: T,
) -> bool {
    let e0 = in_delta(actual.x, expected.x, delta);
    let e1 = in_delta(actual.y, expected.y, delta);
    return e0 && e1;
}

fn spherical_equal<T: CoordFloat + Debug + Display>(
    actual: Coordinate<T>,
    expected: &Coordinate<T>,
    delta: T,
) -> bool {
    let e0 = logitude_equal(actual.x, expected.x, delta);
    let e1 = in_delta(actual.y, expected.y, delta);
    return e0 & e1;
}

fn logitude_equal<T: Float>(actual: T, expected: T, delta: T) -> bool {
    let actual = (actual - expected).abs() % T::from(360f64).unwrap();
    return actual <= delta || actual >= T::from(360f64).unwrap() - delta;
}
