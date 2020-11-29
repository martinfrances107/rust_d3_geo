use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;

/// Point - a single position.
pub struct Point {
    pub coordinate: delaunator::Point,
}

impl DataObject for Point {
    fn to_stream(&self, stream: &mut impl Stream) {
        let g = FeatureGeometry::Point {
            coordinate: self.coordinate.clone(),
        };
        processor(&g, stream);
    }
}
