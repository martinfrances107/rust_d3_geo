// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::Geometry;
use geo::Polygon;
use num_traits::Float;

impl<T: Float> DataObject<T> for Polygon<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        let g = Geometry::Polygon(self.clone());
        processor(&g, stream);
    }
}
