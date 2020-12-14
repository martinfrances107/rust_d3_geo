use std::unimplemented;

use super::line::line;
// use super::polygon::polygon;
use geo::Polygon;

use crate::data_object::feature_geometry::FeatureGeometry;
use crate::stream::Stream;
use geo::Geometry;
use geo::LineString;
use geo::MultiPoint;
use geo::Point;

use num_traits::Float;

pub fn processor<T: Float>(geometry: &Geometry<T>, stream: &mut impl Stream<T>) {
    match geometry {
        Geometry::LineString(ls) => {
            let points: Vec<Point<T>> = ls.points_iter().collect();
            line(&points, stream, 0);
        }
        Geometry::Point(p) => {
            stream.point(p.x(), p.y(), None);
        }
        Geometry::MultiPoint(coordinates) => {
            for c in coordinates {
                stream.point(c.x(), c.y(), None);
            }
        }
        Geometry::Polygon(polygon) => {
            unimplemented!("Must implement Polygon");
        }
        _ => {
            unimplemented!("must implement extra Geometry type.");
        }
    }
}
