use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::CompareIntersection;
use super::Stream;
use super::StreamClone;
// use super::StreamPostClipTrait;
use super::StreamSimpleNode;
// use super::StreamSrc;
use crate::clip::ClipTraitRaw;

#[derive(Clone, Copy, Debug, Default)]
pub struct StreamPostClipNodeStub<T>
where
    T: CoordFloat,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}

impl<T> Stream for StreamPostClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
}

impl<T> StreamClone for StreamPostClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(StreamPostClipNodeStub::default())
    }
}

// TODO:mf must figure out what to stream type to connect in.
// impl<T> StreamPostClipTrait for StreamPostClipNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     type SpostctStream = StreamSrc<T>;
//     fn stream_in(&mut self, _stream: Self::SpostctStream) {
//         // No-op.
//     }
//     fn box_clone(
//         &self,
//     ) -> Box<
//         dyn StreamPostClipTrait<
//             C = Self::C,
//             SctC = Self::SctC,
//             SctOC = Self::SctOC,
//             SctT = Self::SctT,
//             SctCi = Self::SctCi,
//             SctStream = Self::SctStream,
//             SpostctStream = Self::SpostctStream,
//         >,
//     > {
//         panic!("calling box_clone on a stub");
//     }
// }

impl<T> ClipTraitRaw for StreamPostClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    type SctStream = StreamSimpleNode<T>;
    type SctCi = CompareIntersection<T>;
    fn interpolate(
        &self,
        _from: Self::SctOC,
        _to: Self::SctOC,
        _direction: Self::SctT,
        _stream: Self::SctStream,
    ) {
        panic!("Callin interpolate on a stub");
    }

    fn point_visible(&self, _p: Self::SctC, _z: Option<u8>) -> bool {
        panic!("Calling point_visible on a stub");
    }

    // fn stream_resample_in(&mut self, _stream: Self::SpctResample) {
    //     panic!("Calling stream_reample_in on a stub.");
    // }
}
