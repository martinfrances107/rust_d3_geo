use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;

use crate::stream::Stream;
use crate::stream::Streamable;

/// Unit sphere.
#[derive(Clone, Debug)]
pub struct Sphere<T>
// where
//     T: CoordFloat,
{
    pd: PhantomData<T>,
}

impl<T> Default for Sphere<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Sphere {
            pd: PhantomData::default(),
        }
    }
}

impl<T> Streamable for Sphere<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn to_stream<EP, SD: Stream<EP = EP, T = T>>(&self, stream: &mut SD)
    where
        EP: Stream<EP = EP, T = T> + Default,
    {
        stream.sphere();
    }
}
