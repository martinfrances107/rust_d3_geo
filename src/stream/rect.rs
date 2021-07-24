use std::fmt::Display;
use std::ops::AddAssign;

use geo::Rect;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable for Rect<T> {
    // type SC = Coordinate<T>;
    // type SD = Self;
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, _stream: &mut SD) {
        todo!("Do I convert to polygon here?");
    }
}
