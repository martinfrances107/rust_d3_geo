use geo::Line;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

impl<T: CoordFloat + Default + FloatConst> Streamable<T> for Line<T> {
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, _stream: &mut impl Stream<T>) {
        todo!("line start line end?");
    }
}
