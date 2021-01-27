use super::Streamable;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::MultiPolygon;

impl<T: CoordFloat + num_traits::FloatConst> Streamable<T> for MultiPolygon<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for p in self.iter() {
            p.to_stream(stream);
        }
    }
}
