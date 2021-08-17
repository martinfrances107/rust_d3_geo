use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::stream::{Stream, Streamable};

/// Unit sphere.
#[derive(Clone, Debug)]
pub struct Sphere<T>
where
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    pd: PhantomData<T>,
}

impl<T> Default for Sphere<T>
where
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    fn default() -> Self {
        Sphere {
            pd: PhantomData::default(),
        }
    }
}

impl<T> Streamable for Sphere<T>
where
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    type T = T;
    #[inline]
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        stream.sphere();
    }
}
