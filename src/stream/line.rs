use geo::CoordFloat;
use num_traits::FloatConst;

use super::Stream;

use super::Streamable;
use geo::Line;

impl<T: CoordFloat + FloatConst> Streamable<T> for Line<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        stream.point(self.start_point().x(), self.start_point().y(), None);
        stream.point(self.end_point().x(), self.end_point().y(), None);
    }
}
