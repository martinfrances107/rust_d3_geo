use std::fmt::Display;
use std::ops::AddAssign;

use geo::Triangle;

use super::Stream;

use super::Streamable;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable for Triangle<T> {
    // type SC = Coordinate<T>;
    // type SD = Self;
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, _stream: &mut SD) {
        todo!("Do I convert to polygon here?");
    }
}
