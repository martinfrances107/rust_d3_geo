use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use delaunator::Point;
/// MultiPolygon - a multidimensional array of positions forming multiple polygons.
pub struct MultiPolygon {
    pub coordinates: Vec<Vec<Vec<Point>>>,
}

impl DataObject for MultiPolygon {
    fn to_stream(&self, stream: &mut impl Stream) {
        for c in &self.coordinates {
            let g = FeatureGeometry::Polygon {
                coordinates: c.to_vec(),
            };
            processor(&g, stream);
        }
    }
}
