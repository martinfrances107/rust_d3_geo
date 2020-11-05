use super::line::line;
use super::polygon::polygon;

use crate::data_object::FeatureGeometry;
use crate::stream::Stream;

pub fn processor(geometry: &FeatureGeometry, stream: &mut impl Stream) {
    match geometry {
        FeatureGeometry::LineString { coordinates, .. } => {
            line(coordinates, stream, 0);
        }
        FeatureGeometry::Point { coordinate, .. } => {
            stream.point(coordinate.x, coordinate.y, None);
        }
 
        FeatureGeometry::Polygon { coordinates, .. } => {
            polygon(coordinates, stream);
        }
    }
}
