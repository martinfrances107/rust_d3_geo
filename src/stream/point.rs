use geo::{Coordinate, Point};

use super::Stream;

use super::Streamable;

use geo::CoordFloat;
use num_traits::FloatConst;

// Move this to another file.
impl<T: CoordFloat + FloatConst> Streamable for Point<T> {
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, stream: &mut Box<dyn Stream<C = Self::SC>>) {
        // TODO there must be a better way to cast a Point to Coordinate.
        stream.point(
            Coordinate {
                x: self.x(),
                y: self.y(),
            },
            None,
        );
    }
}
