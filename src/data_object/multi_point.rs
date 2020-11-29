use crate::stream::geometry_processor::processor;
use crate::stream::Stream;

use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use delaunator::Point;

pub struct MultiPoint {
    pub coordinates: Vec<Point>,
}

impl DataObject for MultiPoint {
    fn to_stream(&self, stream: &mut impl Stream) {
        for coordinate in &self.coordinates {
            let g = FeatureGeometry::Point {
                coordinate: coordinate.clone(),
            };
            processor(&g, stream);
        }
    }
}
