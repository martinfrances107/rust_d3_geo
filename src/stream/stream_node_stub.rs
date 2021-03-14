use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use super::stream_simple_node_stub::StreamSimpleNode;
use super::Stream;
use super::StreamClone;
use super::StreamInTrait;
// use super::StreamSimpleNode;
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamNodeStub<T>
where
    T: CoordFloat + Default,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}

// impl<T> StreamNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     #[inline]
//     pub fn new() -> StreamSimpleNode<T> {
//         Box::new(StreamNodeStub::<T>::default())
//     }
// }

impl<T> StreamClone for StreamNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(StreamNodeStub::<T>::default())
    }
}
impl<T> Stream for StreamNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamNodeStub<T> where T: CoordFloat + FloatConst + Default {}
