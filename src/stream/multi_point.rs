use crate::stream::Stream;

// use super::feature_geometry::FeatureGeometry;
use super::Streamable;
use geo::CoordFloat;
use geo::MultiPoint;
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> Streamable<T> for MultiPoint<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for p in self.iter() {
            stream.point(p.x(), p.y(), None);
        }
    }
}
