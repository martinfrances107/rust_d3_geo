use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::Stream;
use super::StreamClone;
use super::StreamInTrait;

#[derive(Clone, Copy, Debug, Default)]
pub struct StreamTransformNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}

// impl<T> StreamTransformNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     #[inline]
//     pub fn new() -> StreamTransform<T> {
//         StreamTransform {
//             transform: Box::new(TransformIdentity::default()),
//             stream: Box::new(StreamPreClipNodeStub::default()),
//         }
//     }
// }

impl<T> StreamClone for StreamTransformNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(StreamTransformNodeStub::<T>::default())
    }
}

impl<T> Stream for StreamTransformNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
}

impl<T> StreamInTrait<T> for StreamTransformNodeStub<T> where
    T: CoordFloat + FloatConst + Default + 'static
{
}
