use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use geo::Line;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable for Line<T> {
    // type SD = Self;
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, _stream: &mut SD) {
        todo!("line start line end?");
    }
}
