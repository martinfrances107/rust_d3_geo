use geo::{CoordFloat, Coordinate};
use std::marker::PhantomData;

use num_traits::FloatConst;

// use crate::clip::buffer::ClipBuffer;

// use super::BufferInTrait;
use super::Clean;
use super::CleanEnum;
use super::CompareIntersection;

use super::Stream;
use super::StreamClipLine;
use super::StreamClone;
use super::StreamPreClipTrait;
use super::StreamSimpleNode;
use crate::clip::ClipTraitRaw;
// use crate::projection::resample::resample::Resample;
// use crate::projection::resample::resample_none::ResampleNone;
use crate::projection::resample::ResampleEnum;

#[derive(Clone, Copy, Debug, Default)]
pub struct StreamClipLineNodeStub<T>
where
    T: CoordFloat,
{
    /// Why the Phantom Data is required here...
    ///
    /// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
    /// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
    phantom: PhantomData<T>,
}

// impl<T> StreamClipLineNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     #[inline]
//     pub fn new() -> Box<StreamClipLine> {
//         Rc::new(RefCell::new(Self::default()))
//     }
// }

impl<T> StreamClipLine for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    // #[inline]
    // fn box_clone(&self) -> Box<dyn StreamClipLine<C = Self::C, BitCB = Self::BitCB>> {
    //     panic!("calling box_clone on a stub");
    // }
}

// impl<T> BufferInTrait for StreamClipLineNodeStub<T>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     // type BitSink = Box<dyn StreamPathResult<Out = Option<PathResultEnum<T>>, C = Coordinate<T>>>;
//     type BitCB = ClipBuffer<T>;
//     fn buffer_in(&mut self, _sink: Self::BitCB) {
//         // No-op.
//     }
// }

impl<T> Stream for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type C = Coordinate<T>;
}
impl<T> Clean for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst,
{
    fn clean(&self) -> CleanEnum {
        CleanEnum::NoIntersections
    }
}

impl<T> StreamPreClipTrait for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SpctResample = ResampleEnum<T>;
    // type SPCTstream = StreamSimpleNode<T>;
    fn stream_resample_in(&mut self, _resample: Self::SpctResample) {
        panic!("connecting to a stub!")
    }

    fn box_clone(
        &self,
    ) -> Box<
        dyn StreamPreClipTrait<
            SctC = Self::SctC,
            SctOC = Self::SctOC,
            SctT = Self::SctT,
            SctCi = Self::SctCi,
            SctStream = Self::SctStream,
            SpctResample = Self::SpctResample,
        >,
    > {
        panic!("call box_clone on a stub");
    }
}

impl<T> StreamClone for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}
impl<T> ClipTraitRaw for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
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
