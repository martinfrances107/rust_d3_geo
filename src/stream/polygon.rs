use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::{coords_iter::CoordsIter, Coordinate, Polygon};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::stream_line::stream_line;
use super::Stream;
use super::Streamable;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Streamable<T>
    for Polygon<T>
{
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        stream.polygon_start();

        let e_points: Vec<Coordinate<T>> = self.exterior().coords_iter().collect();
        stream_line(&e_points, stream, 1);

        for i in self.interiors() {
            let line_points: Vec<Coordinate<T>> = i.coords_iter().collect();
            stream_line(&line_points, stream, 1);
        }
        stream.polygon_end();
    }
}
