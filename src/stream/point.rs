use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::{Coordinate, Point};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

// Move this to another file.
impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Streamable
    for Point<T>
{
    // type SC = Coordinate<T>;
    // type C=Coordinate<T>;
    // type SD = Self;
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, stream: &mut SD) {
        // TODO there must be a better way to cast a Point to Coordinate.
        stream.point(
            &Coordinate {
                x: self.x(),
                y: self.y(),
            },
            None,
        );
    }
}
