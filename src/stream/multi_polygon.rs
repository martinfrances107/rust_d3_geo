use super::Streamable;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::Coordinate;
use geo::MultiPolygon;

impl<T: CoordFloat + num_traits::FloatConst> Streamable for MultiPolygon<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<C = Coordinate<T>>) {
        for p in self.iter() {
            p.to_stream(stream);
        }
    }
}
