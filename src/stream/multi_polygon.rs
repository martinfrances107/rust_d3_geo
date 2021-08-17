use std::fmt::Display;

use geo::CoordFloat;
use geo::MultiPolygon;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Streamable;

impl<T: AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable for MultiPolygon<T> {
    type T = T;

    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for p in self.iter() {
            p.to_stream(stream);
        }
    }
}
