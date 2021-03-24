use geo::{Coordinate, Triangle};

use super::Stream;

use super::Streamable;

use geo::CoordFloat;
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> Streamable for Triangle<T> {
    type SC = Coordinate<T>;
    #[inline]
    fn to_stream(&self, _stream: &mut impl Stream<C = Self::SC>) {
        todo!("Do I convert to polygon here?");
    }
}
