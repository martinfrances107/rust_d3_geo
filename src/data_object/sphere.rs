use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;
use std::marker::PhantomData;

use crate::stream::{Stream, Streamable};

// Unit sphere.
#[derive(Clone, Debug)]
pub struct Sphere<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pd: PhantomData<T>,
}

impl<T> Streamable for Sphere<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, stream: &mut SD) {
        stream.sphere();
    }
}
