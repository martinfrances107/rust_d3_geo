use std::fmt::Debug;
use std::fmt::Display;

use geo::CoordFloat;
use geo::Coordinate;

use crate::in_delta::in_delta;
use crate::math::EPSILON;
use crate::Transform;

/// Helper test function.
/// A point projected and then inverted returns to the original location.
///
/// # Panics
///  Will never happen as EPSILON will always be converted into T.
pub fn projection_equal<'a, P, T>(
    projection: &P,
    expected_location: &'a Coordinate<T>,
    expected_point: &'a Coordinate<T>,
    delta_p: Option<T>,
) -> bool
where
    P: Transform<T = T>,
    T: CoordFloat + Display,
{
    let delta = delta_p.map_or_else(|| T::from(EPSILON).unwrap(), |d| d);
    println!("project_equal");
    println!(
        "1) expected location [{:?}, {:?}], expected point [{:?}, {:?}]",
        expected_location.x, expected_location.y, expected_point.x, expected_point.y,
    );
    let actual_location = projection.invert(expected_point);
    let actual_point = projection.transform(expected_location);
    println!(
        "2) actual location {:?}, actual point {:?}",
        actual_location, actual_point,
    );
    planar_equal(&actual_point, expected_point, delta)
        && spherical_equal(&actual_location, expected_location, delta)
}

fn planar_equal<T: CoordFloat + Debug + Display>(
    actual: &Coordinate<T>,
    expected: &Coordinate<T>,
    delta: T,
) -> bool {
    let e0 = in_delta(actual.x, expected.x, delta);
    let e1 = in_delta(actual.y, expected.y, delta);
    e0 && e1
}

fn spherical_equal<T>(actual: &Coordinate<T>, expected: &Coordinate<T>, delta: T) -> bool
where
    T: CoordFloat + Display,
{
    let e0 = logitude_equal(actual.x, expected.x, delta);
    let e1 = in_delta(actual.y, expected.y, delta);
    e0 && e1
}

fn logitude_equal<T>(actual: T, expected: T, delta: T) -> bool
where
    T: CoordFloat,
{
    let actual = (actual - expected).abs() % T::from(360_f64).unwrap();
    actual <= delta || actual >= T::from(360_f64).unwrap() - delta
}
