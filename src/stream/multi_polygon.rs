use super::Streamable;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use geo::MultiPolygon;

impl<T: CoordFloat + Default + num_traits::FloatConst> Streamable<T> for MultiPolygon<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        for p in self.iter() {
            p.to_stream(stream);
        }
    }
}
