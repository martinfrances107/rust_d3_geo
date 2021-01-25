use geo::Polygon;
use geo::{CoordFloat, Point};
use num_traits::FloatConst;

use super::line::line;
use super::Stream;

pub fn polygon<T: CoordFloat + FloatConst>(polygon: &Polygon<T>, stream: &mut impl Stream<T>) {
    stream.polygon_start();

    let e_points: Vec<Point<T>> = polygon.exterior().points_iter().collect();
    line(&e_points, stream, 1);

    for i in polygon.interiors() {
        let line_points: Vec<Point<T>> = i.points_iter().collect();
        line(&line_points, stream, 1);
    }
    stream.polygon_end();
}
