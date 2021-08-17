use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, GeometryCollection};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Streamable;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable
    for GeometryCollection<T>
{
    // type SD = Self;
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for g in self {
            g.to_stream(stream);
        }
    }
}
