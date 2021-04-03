use std::ops::AddAssign;

use geo::{Coordinate, Rect};

use super::Stream;
use super::Streamable;

use geo::CoordFloat;
use num_traits::FloatConst;

impl<T: AddAssign + CoordFloat + Default + FloatConst> Streamable<T> for Rect<T> {
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, _stream: &mut impl Stream<T, C = Self::SC>) {
        todo!("Do I convert to polygon here?");
    }
}
