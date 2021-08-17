use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::stream::{Stream, Streamable};

/// Unit sphere.
#[derive(Clone, Debug)]
pub struct Sphere<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pd: PhantomData<T>,
}

impl<T> Default for Sphere<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    fn default() -> Self {
        Sphere {
            pd: PhantomData::default(),
        }
    }
}

impl<T> Streamable for Sphere<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        stream.sphere();
    }
}
