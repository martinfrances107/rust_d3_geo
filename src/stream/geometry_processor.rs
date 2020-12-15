use std::unimplemented;

use super::line::line;
use super::polygon::polygon;
use crate::stream::Stream;
use geo::Geometry;
use geo::Point;

use num_traits::Float;

pub fn processor<T: Float>(geometry: &Geometry<T>, stream: &mut impl Stream<T>) {
    match geometry {
        Geometry::LineString(ls) => {
            let points: Vec<Point<T>> = ls.points_iter().collect();
            line(&points, stream, 0);
        }
        Geometry::Line(ls) => {
            stream.point(ls.start_point().x(), ls.start_point().y(), None);
            stream.point(ls.end_point().x(), ls.end_point().y(), None);
        }
        Geometry::Point(p) => {
            stream.point(p.x(), p.y(), None);
        }
        Geometry::MultiPoint(coordinates) => {
            for c in coordinates {
                stream.point(c.x(), c.y(), None);
            }
        }
        Geometry::MultiPolygon(mp) => {
            for poly in mp {
                polygon(poly, stream);
            }
        }
        Geometry::Polygon(poly) => {
            polygon(poly, stream);
        }
        Geometry::MultiLineString(mls) => {
            for ls in mls {
                let points: Vec<Point<T>> = ls.points_iter().collect();
                line(&points, stream, 0);
            }
        }
        Geometry::Rect(_) => {
            unimplemented!("Rect");
        }
        Geometry::Triangle(_) => {
            unimplemented!("Triangle");
        }
        Geometry::GeometryCollection(gc) => {
            for g in gc {
                processor(g, stream);
            }
        }
    }
}
