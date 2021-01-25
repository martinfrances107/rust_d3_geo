// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::MultiPolygon;
use geo::{CoordFloat, Geometry};

impl<T: CoordFloat + num_traits::FloatConst> DataObject<T> for MultiPolygon<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for p in self.iter() {
            let g = Geometry::Polygon(p.clone());
            processor(&g, stream);
        }
    }
}
