use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use delaunator::Point;
/// MultiLineString - an array of arrays of positions forming several lines.
pub struct MultiLineString {
    pub coordinates: Vec<Vec<Point>>,
}

impl DataObject for MultiLineString {
    fn to_stream(&self, stream: &mut impl Stream) {
        for coordinate in &self.coordinates {
            let g = FeatureGeometry::LineString {
                coordinates: coordinate.to_vec(),
            };
            processor(&g, stream);
        }
    }
}
