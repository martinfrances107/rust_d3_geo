use std::fmt::Display;
use std::ops::AddAssign;

use super::Streamable;
use crate::stream::Stream;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use geo::MultiPolygon;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable
    for MultiPolygon<T>
{
    // type T = T;
    // type SC = Coordinate<T>;
    // type SD = Self;
    type T = T;
    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, stream: &mut SD) {
        for p in self.iter() {
            p.to_stream(stream);
        }
    }
}
