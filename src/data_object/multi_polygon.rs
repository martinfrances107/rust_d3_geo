// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::Geometry;
use geo::MultiPolygon;
use num_traits::Float;

impl<T: Float> DataObject<T> for MultiPolygon<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for p in self.iter() {
            let g = Geometry::Polygon(p.clone());
            processor(&g, stream);
        }
    }
}
