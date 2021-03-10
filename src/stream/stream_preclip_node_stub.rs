use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::ClipTraitRaw;
// use crate::projection::resample::resample::Resample;
// use crate::projection::resample::resample_none::ResampleNone;
use crate::projection::resample::ResampleEnum;

use super::CompareIntersection;
use super::Stream;
use super::StreamClone;
use super::StreamPreClipTrait;
use super::StreamSimpleNode;

#[derive(Clone, Copy, Debug, Default)]
pub struct StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}
// impl<T> StreamPreClipNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     #[inline]
//     pub fn new() -> Box<Self> {
//         Box::new(Self::default())
//     }
// }

impl<T> StreamPreClipTrait for StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SpctResample = ResampleEnum<T>;
    fn stream_resample_in(&mut self, _stream: Self::SpctResample) {
        // No-op.
    }
    fn box_clone(
        &self,
    ) -> Box<
        dyn StreamPreClipTrait<
            SctC = Self::SctC,
            SctCi = Self::SctCi,
            SctOC = Self::SctOC,
            SctT = Self::SctT,
            SctStream = Self::SctStream,
            SpctResample = Self::SpctResample,
        >,
    > {
        panic!("call box_clone on a stub!");
    }
}

impl<T> Stream for StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
}
impl<T> StreamClone for StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}
impl<T> ClipTraitRaw for StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    type SctStream = StreamSimpleNode<T>;
    type SctCi = CompareIntersection<T>;

    fn point_visible(&self, _p: Self::SctC, _z: Option<u8>) -> bool {
        panic!("Calling point_visible on a stub");
    }

    fn interpolate(
        &self,
        _from: Self::SctOC,
        _to: Self::SctOC,
        _direction: Self::SctT,
        _stream: Self::SctStream,
    ) {
        panic!("Calling interpolate on a stub");
    }
}

// pub type StreamPreClipNode = Box<dyn StreamPreClipTrait>;
// impl<T> StreamClipTrait for StreamPreClipNode {}
// impl<T> Stream for StreamPreClipNode
// where
//     T: CoordFloat + FloatConst,
// {
//
// }

// impl<T> StreamPreClipTrait for StreamPreClipNode
// where
//     T: CoordFloat + FloatConst,
// {
//     #[inline]
//     fn stream_resample_in(&mut self, _stream: ResampleNode<T>) {
//         // No-op.
//     }
// }
