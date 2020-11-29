use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use delaunator::Point;

///  LineString - an array of positions forming a continuous line.
pub struct LineString {
    pub coordinates: Vec<Point>,
}

impl DataObject for LineString {
    fn to_stream(&self, stream: &mut impl Stream) {
        let g = FeatureGeometry::LineString {
            coordinates: self.coordinates.to_vec(),
        };
        processor(&g, stream);
    }
}
