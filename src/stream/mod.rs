mod feature_collection;
mod geometry;
mod geometry_collection;
mod geometry_processor;
mod line;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

pub mod stream_clip_line_node_stub;
pub mod stream_dummy;
pub mod stream_identity;
pub mod stream_node_stub;
pub mod stream_path_result_identity;
pub mod stream_path_result_node_stub;
pub mod stream_postclip_node_stub;
pub mod stream_preclip_node_stub;
pub mod stream_resample_node_stub;
pub mod stream_transform_node_stub;

use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use crate::clip::BufferInTrait;
use crate::clip::ClipTraitRaw;
use crate::path::PathResult;
use crate::path::PathResultEnum;

/// Applies to DataObject's
pub trait Streamable {
    type SC;
    fn to_stream(&self, stream: &mut impl Stream<C = Self::SC>);
}

// Takes a line and cuts into visible segments. Return values used for polygon
// clipPing: 0 - there were intersections or the line was empty; 1 - no
// intersections 2 - there were intersections, and the first and last segments
// should be rejoined.
#[derive(Debug, Clone, Copy)]
pub enum CleanEnum {
    IntersectionsOrEmpty,
    NoIntersections,
    IntersectionsRejoin,
}

pub trait Clean {
    /// A clip trait.
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    fn clean(&self) -> CleanEnum {
        panic!("must related code to enum");
    }
}

pub trait StreamClone {
    type RetType;
    fn box_clone(&self) -> Self::RetType;
}

#[derive(Clone, Default)]
pub struct StreamSourceDummy<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

#[derive(Clone)]
pub enum StreamSrc<T>
where
    T: CoordFloat,
{
    SRC(StreamSourceDummy<T>),
}

pub trait Stream {
    type C;
    fn point(&mut self, _p: Self::C, _m: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}

impl<T> Stream for StreamSrc<T>
where
    T: CoordFloat + std::fmt::Debug,
{
    type C = Coordinate<T>;
}

pub trait StreamInTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, _stream: Box<dyn Stream<C = Coordinate<T>>>) {}
}

pub trait StreamClipLine: Stream + Clean {
    // fn box_clone(&self) -> Box<dyn StreamClipLine<C = Self::C, BitCB = Self::BitCB>>;
}

pub trait StreamClean<T>: Stream + Clean
where
    T: CoordFloat + FloatConst,
{
}

pub trait StreamPathResult: Stream + PathResult {
    fn box_clone(&self) -> Box<dyn StreamPathResult<C = Self::C, Out = Self::Out>>;
}

pub trait StreamPostClipTrait: ClipTraitRaw + Stream {
    type SpostctStream;
    fn stream_in(&mut self, _stream: Self::SpostctStream) {
        // No-op.
    }
    fn box_clone(
        &self,
    ) -> Box<
        dyn StreamPostClipTrait<
            SpostctStream = Self::SpostctStream,
            C = Self::C,
            SctC = Self::SctC,
            SctOC = Self::SctOC,
            SctT = Self::SctT,
            SctCi = Self::SctCi,
            SctStream = Self::SctStream,
        >,
    >;
}

// pub type StreamClipLineNode<T> =
//     Rc<RefCell<dyn StreamClipLine<T, C = Coordinate<T>>>>;
// impl<T> StreamClone for StreamClipLineNode<T> where T: CoordFloat + FloatConst {

//     type C = Coordinate<T>;
//     #[inline]
//     fn box_clone(&self) -> Box<dyn Stream<C = Coordinate<T>>> {
//         Box::new(*self.clone())
//     }
// }

// impl<T> Stream for StreamClipLineNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//
// }
// impl<T> StreamInTrait<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}
// impl<T> BufferInTrait<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}

// impl<T> BufferInTrait<T> for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst {}

// pub trait StreamClone {
//     type ScC;
//     fn box_clone(&self) -> Box<dyn Self>;
// }

// use crate::projection::resample::resample;

// pub type StreamResampleNode<T> = Box<dyn StreamResampleTrait<SRTsci = StreamPostClipNode<T>>>;

// impl<T: 'static> StreamClone
//     for Box<dyn StreamResampleTrait<C = Coordinate<T>, SRTsci = StreamPostClipNode<T>>>
// where
//     T: CoordFloat + FloatConst,
// {
//     type C = Coordinate<T>;
//     fn box_clone(
//         &self,
//     ) -> Box<dyn StreamResampleTrait<C = Coordinate<T>, SRTsci = StreamPostClipNode<T>>> {
//         Box::new(*self.clone())
//     }
// }

/// Ci CompareIntersections param type
/// See StreamClipTrait.
#[derive(Clone, Debug, Default)]
pub struct CompareIntersection<T: CoordFloat>
where
    T: CoordFloat + FloatConst,
{
    x: Coordinate<T>,
}

/// Node - holds state associated with the input/output of a StreamProcessor.
/// Something that can be cloned and mutated.

pub type StreamSimpleNode<T> = Box<dyn Stream<C = Coordinate<T>>>;
// impl<T> Stream for StreamSimpleNode<T> where T: CoordFloat + FloatConst {}
// impl<T> StreamInTrait<T> for StreamSimpleNode<T> where T: CoordFloat + FloatConst {}
// impl<T> StreamSimpleNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     fn new() -> StreamSimpleNode<T> {
//         Rc::new(RefCell::new(StreamDummy::default()))
//     }
// }

// pub type StreamPathResultNode<T> =
// //     Rc<RefCell<dyn StreamPathResult<T, C = Coordinate<T>>>>;
// impl<T> StreamClone for StreamPathResultNode<T> where T: CoordFloat + FloatConst {
//     type C = Coordinate<T>;
// }
// impl<T> Stream for StreamPathResultNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//
// }
// impl<T> StreamInTrait<T> for StreamPathResultNode<T> where T: CoordFloat + FloatConst {}

pub trait StreamPreClipTrait: ClipTraitRaw {
    type SpctResample;
    fn stream_resample_in(&mut self, stream: Self::SpctResample);

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
    >;
}

// pub type StreamPostClipNode<T> = Box<dyn StreamPostClipTrait<>>;
// impl<T> StreamPostClipTrait<T> for StreamPostClipNode<T> where T: CoordFloat + FloatConst {}
// impl<T> StreamClipTrait for StreamPostClipNode<T> where T: CoordFloat + FloatConst {}
// impl<T> Stream for StreamPostClipNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//
// }

// pub type StreamTransformNode<T> = Box<StreamTransform<T>>;
// impl<T> StreamClone for StreamTransformNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     type C = Coordinate<T>;
//     #[inline]
//     fn box_clone(&self) -> Box<dyn Stream<C = Coordinate<T>>> {
//         Box::new(self.clone())
//     }
// }
// impl<T> Stream for StreamTransformNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//
// }
// impl<T> StreamPreclipIn<T> for StreamTransformNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     #[inline]
//     fn stream_preclip_in(
//         &mut self,
//         _stream: Box<
//             dyn StreamPreClipTrait<
//                 C = Coordinate<T>,
//
//                 SctC = Coordinate<T>,
//                 SctT = T,
//                 SctOC = Option<Coordinate<T>>,
//                 SctCi = CompareIntersection<T>,
//                 SctStream = dyn Stream<C = Coordinate<T>>,
//                 SPCTstream = dyn Stream<C = Coordinate<T>>,
//             >,
//         >,
//     ) {
//         // No-op.
//     }
// }
