use geo::Point;

use super::Stream;

use super::Streamable;

use geo::CoordFloat;
use num_traits::FloatConst;

// Move this to another file.
impl<T: CoordFloat + FloatConst> Streamable<T> for Point<T> {
    #[inline]
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        stream.point(self.x(), self.y(), None);
    }
}
