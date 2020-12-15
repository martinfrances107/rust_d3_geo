use geo::Point;
use geo::Polygon;
use num_traits::Float;

use super::line::line;
use super::Stream;

pub fn polygon<T: Float>(polygon: &Polygon<T>, stream: &mut impl Stream<T>) {
    stream.polygon_start();

    let e_points: Vec<Point<T>> = polygon.exterior().points_iter().collect();
    line(&e_points, stream, 1);

    for i in polygon.interiors() {
        let line_points: Vec<Point<T>> = i.points_iter().collect();
        line(&line_points, stream, 1);
    }
    stream.polygon_end();
}
