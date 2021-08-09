use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate, LineString};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::length::Stream;

pub fn distance<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst>(
    a: &Coordinate<T>,
    b: &Coordinate<T>,
) -> T {
    // TODO consider making object static outside of distance.
    // It does not need to be created each time.
    let object = LineString(vec![(*a).clone().into(), (*b).clone().into()]);

    return Stream::<T>::calc(&object);
}
