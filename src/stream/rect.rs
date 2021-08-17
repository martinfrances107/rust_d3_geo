use std::fmt::Display;

use geo::CoordFloat;
use geo::Rect;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

impl<T: AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable for Rect<T> {
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, _stream: &mut SD) {
        todo!("Do I convert to polygon here?");
    }
}
