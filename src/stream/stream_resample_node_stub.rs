use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::Stream;
use super::StreamClone;

#[derive(Clone, Copy, Debug, Default)]
pub struct StreamResampleNodeStub<T>
where
    T: CoordFloat + Default + 'static,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}

// impl<T> StreamResampleNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     #[inline]
//     fn new() -> Box<dyn StreamResampleTrait<SRTsci = StreamPostClipNode<T>>> {
//         Box::new(Self::default())
//     }
// }

// impl<T> StreamClone for StreamResampleNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     type C = Coordinate<T>;
//     fn box_clone(
//         &self,
//     ) -> Box<dyn StreamResampleTrait<C = Coordinate<T>, SRTsci = StreamPostClipNode<T>>> {
//         Box::new(Self::default())
//     }
// }

// impl<T> StreamResampleTrait for StreamResampleNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     type SRTsci = StreamPostClipNode<T>;
//     fn stream_postclip_in(&mut self, _stream: StreamPostClipNode<T>) {
//         // No-op.
//     }
// }
impl<T> StreamClone for StreamResampleNodeStub<T>
where
    T: CoordFloat + FloatConst + Default,
{
    type RetType = Box<StreamResampleNodeStub<T>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(StreamResampleNodeStub::default())
    }
}
impl<T> Stream for StreamResampleNodeStub<T>
where
    T: CoordFloat + FloatConst + Default,
{
    type C = Coordinate<T>;
}
