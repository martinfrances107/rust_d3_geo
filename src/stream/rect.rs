use std::fmt::Display;
use std::ops::AddAssign;

use geo::{Coordinate, Rect};

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::Stream;
use super::Streamable;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Streamable<T>
    for Rect<T>
{
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, _stream: &mut impl Stream<T, C = Self::SC>) {
        todo!("Do I convert to polygon here?");
    }
}
