use std::fmt::Display;
use std::ops::AddAssign;

use geo::Coordinate;
use geo::{CoordFloat, GeometryCollection};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Streamable;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Streamable<T>
    for GeometryCollection<T>
{
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        for g in self {
            g.to_stream(stream);
        }
    }
}
