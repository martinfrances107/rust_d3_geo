use core::fmt::Debug;
use core::marker::PhantomData;

use geo::CoordFloat;

use crate::stream::Stream;
use crate::stream::Streamable;

/// Unit sphere.
#[derive(Clone, Debug, Default)]
pub struct Sphere<T> {
    pd: PhantomData<T>,
}

impl<T> Streamable for Sphere<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<EP, SD: Stream<EP = EP, T = T>>(&self, stream: &mut SD) {
        stream.sphere();
    }
}
