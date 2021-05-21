use std::fmt::Display;
use std::marker::PhantomData;

use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::stream::{Stream, Streamable};
// Unit sphere.
#[derive(Clone, Debug)]
pub struct Sphere {
    // phantom: PhantomData<T>,
}

impl<T> Streamable<T> for Sphere
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        stream.sphere();
    }
}
