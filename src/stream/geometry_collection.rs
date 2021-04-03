use std::ops::AddAssign;

use geo::Coordinate;
use geo::{CoordFloat, GeometryCollection};

use crate::stream::Stream;

use super::Streamable;

impl<T: AddAssign + CoordFloat + Default + num_traits::FloatConst> Streamable<T>
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
