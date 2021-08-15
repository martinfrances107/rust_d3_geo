use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::in_delta::in_delta;
use crate::Transform;

pub fn projection_equal<
    'a,
    P: Transform,
    T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst + Debug + Display + Default,
>(
    projection: &P,
    expected_location: &'a Coordinate<T>,
    expected_point: &'a Coordinate<T>,
    delta_p: Option<T>,
) -> bool
where
    P: Transform<C = Coordinate<T>>,
{
    let delta = match delta_p {
        Some(d) => d,
        None => T::from(1e-6_f64).unwrap(),
    };
    println!("project_equal");
    println!(
        "1) expected location [{:?}, {:?}], expected point [{:?}, {:?}]",
        expected_location.x, expected_location.y, expected_point.x, expected_point.y,
    );
    let actual_location = projection.invert(&expected_point);
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

fn spherical_equal<T: CoordFloat + Debug + Display>(
    actual: &Coordinate<T>,
    expected: &Coordinate<T>,
    delta: T,
) -> bool {
    let e0 = logitude_equal(actual.x, expected.x, delta);
    let e1 = in_delta(actual.y, expected.y, delta);
    e0 && e1
}

fn logitude_equal<T: Float>(actual: T, expected: T, delta: T) -> bool {
    let actual = (actual - expected).abs() % T::from(360_f64).unwrap();
    actual <= delta || actual >= T::from(360_f64).unwrap() - delta
}
