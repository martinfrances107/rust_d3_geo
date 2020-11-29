use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use delaunator::Point;

/// Polygon - an array of arrays of positions forming a polygon (possibly with holes).
pub struct Polygon {
    pub coordinates: Vec<Vec<Point>>,
}

impl DataObject for Polygon {
    fn to_stream(&self, stream: &mut impl Stream) {
        let g = FeatureGeometry::Polygon {
            coordinates: self.coordinates.to_vec(),
        };
        processor(&g, stream);
    }
}
