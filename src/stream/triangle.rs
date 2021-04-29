use std::fmt::Display;
use std::ops::AddAssign;

use geo::{Coordinate, Triangle};

use super::Stream;

use super::Streamable;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Streamable<T>
    for Triangle<T>
{
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, _stream: &mut impl Stream<T, C = Self::SC>) {
        todo!("Do I convert to polygon here?");
    }
}
