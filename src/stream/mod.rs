mod feature_collection;
mod geometry;
mod geometry_collection;
mod line;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

mod geometry_processor;

use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::Float;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::clip::BufferInTrait;
use crate::path::PathResult;
use crate::path::PathResultEnum;
use crate::projection::resample::ResampleNode;
use crate::projection::stream_transform::StreamPreclipIn;
// use crate::stream::StreamPathResult;
use crate::clip::buffer::ClipBuffer;
use crate::{projection::stream_transform::StreamTransform, TransformIdentity};
// pub type StreamProcessor<T> = Box<dyn Fn(StreamNode<T>) -> StreamNode<T>>;
// pub type StreamPathResultToStreamProcessor<T> =
//     Box<dyn Fn(StreamPathResultNode<T>) -> StreamNode<T>>;
// pub type StreamPathResultToCleanProcessor<T> =
//     Box<dyn Fn(StreamPathResultNode<T>) -> StreamCleanNode<T>>;

/// A Stub acts as a black hole.
/// A StreamIdentity acts as a 'pass through' node.
pub struct StreamIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    stream: Box<dyn Stream<ScC = Coordinate<T>>>,
}

impl<T> Default for StreamIdentity<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn default() -> Self {
        Self {
            stream: Box::new(StreamDummy::default()),
        }
    }
}

impl<T> StreamClone for StreamIdentity<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type ScC = Coordinate<T>;
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(StreamIdentity::<T>::default())
    }
}

impl<T> Stream for StreamIdentity<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        self.stream.point(p, m);
    }
    #[inline]
    fn sphere(&mut self) {
        self.stream.sphere();
    }
    #[inline]
    fn line_start(&mut self) {
        self.stream.line_start();
    }
    #[inline]
    fn line_end(&mut self) {
        self.stream.line_end();
    }
    #[inline]
    fn polygon_start(&mut self) {
        self.stream.polygon_start();
    }
    #[inline]
    fn polygon_end(&mut self) {
        self.stream.polygon_end();
    }
}

impl<T> StreamInTrait<T> for StreamIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, stream: Box<dyn Stream<ScC = Coordinate<T>>>) {
        self.stream = stream;
    }
}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Default, Debug)]
pub struct StreamPathResultIdentity<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}
impl<T> StreamPathResult for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    // type Out = Option<PathResultEnum<T>>;
}
impl<T> StreamClone for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}
impl<T> Stream for StreamPathResultIdentity<T> where T: CoordFloat + FloatConst + 'static {}
impl<T> StreamInTrait<T> for StreamPathResultIdentity<T> where T: CoordFloat + FloatConst {}
impl<T> PathResult for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Self::Out {
        None
    }
}

/// Applies to DataObject's
pub trait Streamable {
    type SC;
    fn to_stream(&self, stream: impl Stream<ScC = Self::SC>);
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
    fn clean(&self) -> CleanEnum;
}

pub trait StreamClone {
    type ScC;
    fn clone_box(&self) -> Box<dyn Stream<ScC = Self::ScC>>;
}

pub trait Stream: StreamClone {
    // type C;
    fn point(&mut self, _p: Self::ScC, _m: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}

pub trait StreamInTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, _stream: Box<dyn Stream<ScC = Coordinate<T>>>) {}
}

pub trait StreamClipLine: Stream + Clean + BufferInTrait {}

pub trait StreamClean<T>: Stream + Clean
where
    T: CoordFloat + FloatConst,
{
}

pub trait StreamPathResult: Stream + PathResult {}

// pub type StreamClipLineNode<T> =
//     Rc<RefCell<dyn StreamClipLine<T, ScC = Coordinate<T>>>>;
// impl<T> StreamClone for StreamClipLineNode<T> where T: CoordFloat + FloatConst {

//     type ScC = Coordinate<T>;
//     #[inline]
//     fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
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

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamClipLineNodeStub<T>
where
    T: CoordFloat,
{
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
impl<T> StreamClipLine for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst + 'static {}
impl<T> BufferInTrait for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    // type BitSink = Box<dyn StreamPathResult<Out = Option<PathResultEnum<T>>, ScC = Coordinate<T>>>;
    type BitCB = ClipBuffer<T>;
    fn buffer_in(&mut self, _sink: Self::BitCB) {
        // No-op.
    }
}
impl<T> Stream for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst + 'static {}
impl<T> Clean for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst,
{
    fn clean(&self) -> CleanEnum {
        CleanEnum::NoIntersections
    }
}
// impl<T> BufferInTrait<T> for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst {}
impl<T> StreamPreClipTrait for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type SpctResample = ResampleNode<T>;
    // type SPCTstream = StreamSimpleNode<T>;
    fn stream_resample_in(&mut self, _resample: Self::SpctResample) {
        // Drop input.
    }
}

impl<T> StreamClone for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}
impl<T> StreamClipTrait for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    type SctStream = StreamSimpleNode<T>;
    type SctCi = CompareIntersection<T>;

    fn point_visible(&self, p: Self::ScC, _z: Option<u8>) -> bool {
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

// pub trait StreamClone {
//     type ScC;
//     fn clone_box(&self) -> Box<dyn Self>;
// }

// use crate::projection::resample::resample;

// pub type StreamResampleNode<T> = Box<dyn StreamResampleTrait<SRTsci = StreamPostClipNode<T>>>;

// impl<T: 'static> StreamClone
//     for Box<dyn StreamResampleTrait<ScC = Coordinate<T>, SRTsci = StreamPostClipNode<T>>>
// where
//     T: CoordFloat + FloatConst,
// {
//     type ScC = Coordinate<T>;
//     fn clone_box(
//         &self,
//     ) -> Box<dyn StreamResampleTrait<ScC = Coordinate<T>, SRTsci = StreamPostClipNode<T>>> {
//         Box::new(*self.clone())
//     }
// }

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamResampleNodeStub<T>
where
    T: CoordFloat + Default + 'static,
{
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
//     type ScC = Coordinate<T>;
//     fn clone_box(
//         &self,
//     ) -> Box<dyn StreamResampleTrait<ScC = Coordinate<T>, SRTsci = StreamPostClipNode<T>>> {
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
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(StreamResampleNodeStub::default())
    }
}
impl<T> Stream for StreamResampleNodeStub<T> where T: CoordFloat + FloatConst + Default {}

/// Ci CompareIntersections param type
/// See StreamClipTrait.
#[derive(Clone, Debug, Default)]
pub struct CompareIntersection<T: CoordFloat>
where
    T: CoordFloat + FloatConst,
{
    x: Coordinate<T>,
}
pub trait StreamClipTrait: Stream // where
//     T: CoordFloat + FloatConst,
{
    type SctOC;
    type SctT: CoordFloat + FloatConst;
    type SctStream;
    type SctCi;
    fn interpolate(
        &self,
        _from: Self::SctOC,
        _to: Self::SctOC,
        _direction: Self::SctT,
        _stream: Self::SctStream,
    );

    fn point_visible(&self, _p: Self::ScC, _z: Option<u8>) -> bool;

    // fn clip_line(&self, stream: StreamPathResultNode<T>) -> StreamCleanNode<T>;
    // Intersections are sorted along the clip edge. For both antimeridian cutting
    // and circle clipPIng, the same comparison is used.
    fn compare_intersection(&self, a: Self::SctCi, b: Self::SctCi) -> Self::SctT {
        let a_dashed = a.x;
        let part1 = match a_dashed.x < Self::SctT::zero() {
            true => a_dashed.y - Self::SctT::FRAC_PI_2() - Self::SctT::epsilon(),
            false => Self::SctT::FRAC_PI_2() - a_dashed.y,
        };
        let b_dashed = b.x;
        let part2 = match b_dashed.x < Self::SctT::zero() {
            true => b_dashed.y - Self::SctT::FRAC_PI_2() - Self::SctT::epsilon(),
            false => Self::SctT::FRAC_PI_2() - b_dashed.y,
        };

        return part1 - part2;
    }
    // fn clone_box(&self) -> Box<Clip> {
    //     Box::new(self.clone);
    // }
}

/// Node - holds state associated with the input/output of a StreamProcessor.
/// Something that can be cloned and mutated.

pub type StreamSimpleNode<T> = Box<dyn Stream<ScC = Coordinate<T>>>;
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
// //     Rc<RefCell<dyn StreamPathResult<T, ScC = Coordinate<T>>>>;
// impl<T> StreamClone for StreamPathResultNode<T> where T: CoordFloat + FloatConst {
//     type ScC = Coordinate<T>;
// }
// impl<T> Stream for StreamPathResultNode<T>
// where
//     T: CoordFloat + FloatConst,
// {
//
// }
// impl<T> StreamInTrait<T> for StreamPathResultNode<T> where T: CoordFloat + FloatConst {}

pub trait StreamPreClipTrait: StreamClipTrait {
    type SpctResample;
    fn stream_resample_in(&mut self, stream: Self::SpctResample);
}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
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
    type SpctResample = ResampleNode<T>;
    fn stream_resample_in(&mut self, _stream: Self::SpctResample) {
        // No-op.
    }
}
impl<T> Stream for StreamPreClipNodeStub<T> where T: CoordFloat + FloatConst + Default + 'static {}
impl<T> StreamClone for StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}
impl<T> StreamClipTrait for StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type SctOC = Option<Coordinate<T>>;

    type SctT = T;

    type SctStream = StreamSimpleNode<T>;

    type SctCi = CompareIntersection<T>;

    //    type SctCi;
    fn point_visible(&self, _p: Self::ScC, _z: Option<u8>) -> bool {
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

pub trait StreamPostClipTrait: StreamClipTrait {
    fn stream_in(&mut self, _stream: StreamSimpleNode<Self::SctT>);
}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamPostClipNodeStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}
// impl<T> StreamPostClipNodeStub<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     #[inline]
//     pub fn new() -> Box<
//         dyn StreamPostClipTrait<
//
//             SctC = Coordinate<T>,
//             SctT = T,
//             SctOC = Option<Coordinate<T>>,
//             SctCi = CompareIntersection<T>,
//             SctStream = dyn Stream<ScC = Coordinate<T>>,
//         >,
//     > {
//         Box::new(Self::default())
//     }
// }
// impl<T> StreamClone for StreamPostClipNodeStub<T> where T: CoordFloat + FloatConst {}
impl<T> StreamPostClipTrait for StreamPostClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn stream_in(&mut self, _stream: StreamSimpleNode<Self::SctT>) {}
}

impl<T> Stream for StreamPostClipNodeStub<T> where T: CoordFloat + FloatConst + Default + 'static {}
impl<T> StreamClone for StreamPostClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(StreamPostClipNodeStub::default())
    }
}
impl<T> StreamClipTrait for StreamPostClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
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

    fn point_visible(&self, _p: Self::ScC, _z: Option<u8>) -> bool {
        panic!("Calling point_visible on a stub");
    }

    // fn stream_resample_in(&mut self, _stream: Self::SpctResample) {
    //     panic!("Calling stream_reample_in on a stub.");
    // }
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
//     type ScC = Coordinate<T>;
//     #[inline]
//     fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
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
//                 ScC = Coordinate<T>,
//
//                 SctC = Coordinate<T>,
//                 SctT = T,
//                 SctOC = Option<Coordinate<T>>,
//                 SctCi = CompareIntersection<T>,
//                 SctStream = dyn Stream<ScC = Coordinate<T>>,
//                 SPCTstream = dyn Stream<ScC = Coordinate<T>>,
//             >,
//         >,
//     ) {
//         // No-op.
//     }
// }

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamDummy<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}
impl<T> StreamClipTrait for StreamDummy<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    type SctStream = StreamSimpleNode<T>;
    type SctCi = CompareIntersection<T>;

    fn point_visible(&self, p: Self::ScC, _z: Option<u8>) -> bool {
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
impl<T> StreamPreClipTrait for StreamDummy<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type SpctResample = ResampleNode<T>;
    fn stream_resample_in(&mut self, stream: Self::SpctResample) {
        // Drop input.
    }
}

impl<T> StreamClone for StreamDummy<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}
impl<T> Stream for StreamDummy<T> where T: CoordFloat + FloatConst + 'static {}
impl<T> StreamInTrait<T> for StreamDummy<T> where T: CoordFloat + FloatConst {}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamTransformNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
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
    type ScC = Coordinate<T>;
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(StreamTransformNodeStub::<T>::default())
    }
}
impl<T> Stream for StreamTransformNodeStub<T> where T: CoordFloat + FloatConst + Default + 'static {}
impl<T> StreamInTrait<T> for StreamTransformNodeStub<T> where
    T: CoordFloat + FloatConst + Default + 'static
{
}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamNodeStub<T>
where
    T: CoordFloat + Default,
{
    phantom: PhantomData<T>,
}

impl<T> StreamNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    pub fn new() -> StreamSimpleNode<T> {
        Box::new(StreamNodeStub::<T>::default())
    }
}
impl<T> StreamClone for StreamNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(StreamNodeStub::<T>::default())
    }
}
impl<T> Stream for StreamNodeStub<T> where T: CoordFloat + FloatConst + Default + 'static {}
impl<T> StreamInTrait<T> for StreamNodeStub<T> where T: CoordFloat + FloatConst + Default {}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamPathResultNodeStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}
impl<T> PathResult for StreamPathResultNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type Out = Option<PathResultEnum<T>>;
    fn result(&mut self) -> Self::Out {
        None
    }
}
impl<T> StreamPathResult for StreamPathResultNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    // #[inline]
    // pub fn new() -> StreamPathResultNode<T> {
    //     Rc::new(RefCell::new(StreamPathResultIdentity::default()))
    // }
}
impl<T> StreamClone for StreamPathResultNodeStub<T>
where
    T: CoordFloat + 'static,
{
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}
impl<T> Stream for StreamPathResultNodeStub<T> where T: CoordFloat + 'static {}
impl<T> StreamInTrait<T> for StreamPathResultNodeStub<T> where T: CoordFloat + FloatConst {}
