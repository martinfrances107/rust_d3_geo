use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::Stream;

use super::Streamable;
use geo::Line;

impl<T: CoordFloat + FloatConst> Streamable for Line<T> {
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, _stream: &mut impl Stream) {
        todo!("line start line end?");
    }
}
