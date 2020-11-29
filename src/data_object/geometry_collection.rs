use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
/// GeometryCollection - an array of geometry objects.
pub struct GeometryCollection {
    pub geometries: Vec<FeatureGeometry>,
}

impl DataObject for GeometryCollection {
    fn to_stream(&self, stream: &mut impl Stream) {
        for g in &self.geometries {
            processor(&g, stream);
        }
    }
}
