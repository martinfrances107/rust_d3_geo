use crate::stream::geometry_processor::processor;
use crate::stream::Stream;

// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use geo::MultiPoint;
use geo::{CoordFloat, Geometry};
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> DataObject<T> for MultiPoint<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for p in self.iter() {
            let g = Geometry::Point(*p);
            processor(&g, stream);
        }
    }
}
