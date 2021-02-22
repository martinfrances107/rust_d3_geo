use super::Streamable;
use crate::stream::Stream;
use geo::Coordinate;
use geo::{CoordFloat, GeometryCollection};

impl<T: CoordFloat + num_traits::FloatConst> Streamable for GeometryCollection<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<C = Self::SC>) {
        for g in self {
            g.to_stream(stream);
        }
    }
}
