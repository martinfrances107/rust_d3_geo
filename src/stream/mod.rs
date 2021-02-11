pub mod feature_collection;
pub mod geometry;
pub mod geometry_collection;
mod geometry_processor;
pub mod line;
pub mod line_string;
pub mod multi_line_string;
pub mod multi_point;
pub mod multi_polygon;
pub mod point;
pub mod polygon;
use crate::{projection::stream_transform::StreamTransform, TransformIdentity};
use std::cell::RefCell;
use std::rc::Rc;
// use crate::path::PathResult;
// use crate::transform_stream::StreamPathResultIdentity;
use geo::{CoordFloat, Coordinate};
// use num_traits::FloatConst;

// use crate::stream::StreamCleanNode;

// use crate::stream::{Stream, StreamPathResultNode};
use crate::clip::BufferInTrait;
use crate::path::PathResult;

// use geo::CoordFloat;
use num_traits::FloatConst;

// pub type StreamProcessor<T> = Box<dyn Fn(StreamNode<T>) -> StreamNode<T>>;
// pub type StreamPathResultToStreamProcessor<T> =
//     Box<dyn Fn(StreamPathResultNode<T>) -> StreamNode<T>>;
// pub type StreamPathResultToCleanProcessor<T> =
//     Box<dyn Fn(StreamPathResultNode<T>) -> StreamCleanNode<T>>;

// impl<T> Stream<T> for ClipNode<T> where T: CoordFloat + FloatConst {}
// impl<T> StreamSimple<T> for ClipNode<T> where T: CoordFloat + FloatConst {}
#[derive(Clone, Default, Debug)]
pub struct StreamIdentity {}
impl<T> Stream<T> for StreamIdentity where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamIdentity where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamIdentity where T: CoordFloat + FloatConst {}

// impl<T> StreamSimpleNode<T> for StreamIdentity where T: CoordFloat + FloatConst {}
#[derive(Clone, Default, Debug)]
pub struct StreamPathResultIdentity {}
impl<T> StreamPathResult<T> for StreamPathResultIdentity where T: CoordFloat + FloatConst {}
impl<T> Stream<T> for StreamPathResultIdentity where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamPathResultIdentity where T: CoordFloat + FloatConst {}
impl<T> PathResult<T> for StreamPathResultIdentity where T: CoordFloat + FloatConst {}

/// Applies to DataObject's
pub trait Streamable<T: CoordFloat + FloatConst> {
    fn to_stream(&self, stream: &mut impl Stream<T>);
}

// Takes a line and cuts into visible segments. Return values used for polygon
// clipPing: 0 - there were intersections or the line was empty; 1 - no
// intersections 2 - there were intersections, and the first and last segments
// should be rejoined.
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

pub trait Stream<T>
where
    T: CoordFloat + FloatConst,
{
    fn point(&mut self, p: Coordinate<T>, _m: Option<u8>) {}
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
    fn stream_in(&mut self, _stream: StreamSimpleNode<T>) {}
}

/// Bare bones definition - no extra methods attached.
pub trait StreamSimple<T>: Stream<T>
where
    T: CoordFloat + FloatConst,
{
}

pub trait StreamClipLine<T>: Stream<T> + Clean + BufferInTrait<T>
where
    T: CoordFloat + FloatConst,
{
}

pub trait StreamClean<T>: Stream<T> + Clean
where
    T: CoordFloat + FloatConst,
{
}

pub trait StreamPathResult<T>: Stream<T> + PathResult<T>
where
    T: CoordFloat + FloatConst,
{
}

// begin
// pub type Stream
//end
pub type StreamClipLineNode<T> = Rc<RefCell<Box<dyn StreamClipLine<T>>>>;
impl<T> Stream<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}
impl<T> BufferInTrait<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}
pub struct StreamClipLineNodeStub {}
impl StreamClipLineNodeStub {
    #[inline]
    pub fn new<T>() -> StreamClipLineNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(Self {})))
    }
}
impl<T> StreamClipLine<T> for StreamClipLineNodeStub where T: CoordFloat + FloatConst {}
impl<T> Stream<T> for StreamClipLineNodeStub where T: CoordFloat + FloatConst {}
impl Clean for StreamClipLineNodeStub {
    fn clean(&self) -> CleanEnum {
        CleanEnum::NoIntersections
    }
}
impl<T> BufferInTrait<T> for StreamClipLineNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamPreClipTrait<T> for StreamClipLineNodeStub where T: CoordFloat + FloatConst {}

impl<T> StreamClipTrait<T> for StreamClipLineNodeStub where T: CoordFloat + FloatConst {}

pub trait StreamResampleTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_postclip_in(&mut self, stream_clip_in: StreamPostClipNode<T>);
}
pub type StreamResampleNode<T> = Rc<RefCell<Box<dyn StreamResampleTrait<T>>>>;
impl<T> StreamResampleTrait<T> for StreamResampleNode<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_postclip_in(&mut self, _stream_clip_in: StreamPostClipNode<T>) {
        // No-op
    }
}

pub struct StreamResampleNodeStub {}
impl StreamResampleNodeStub {
    #[inline]
    fn new<T>() -> StreamResampleNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(Self {})))
    }
}
impl<T> StreamResampleTrait<T> for StreamResampleNodeStub
where
    T: CoordFloat + FloatConst,
{
    fn stream_postclip_in(&mut self, _stream: StreamPostClipNode<T>) {
        // No-op
    }
}
impl<T> Stream<T> for StreamResampleNodeStub where T: CoordFloat + FloatConst {}

/// Ci CompareIntersections param type
/// See StreamClipTrait.
#[derive(Clone, Debug)]
pub struct Ci<T: CoordFloat>
where
    T: CoordFloat + FloatConst,
{
    x: Coordinate<T>,
}
pub trait StreamClipTrait<T>: Stream<T>
where
    T: CoordFloat + FloatConst,
{
    fn interpolate(
        &self,
        _from: Option<Coordinate<T>>,
        _to: Option<Coordinate<T>>,
        _direction: T,
        _stream: StreamSimpleNode<T>,
    ) {
        panic!("This must be overriden");
    }

    #[inline]
    fn point_visible(&self, _p: Coordinate<T>, _z: Option<u8>) -> bool {
        panic!("This must be overriden");
    }

    // fn clip_line(&self, stream: StreamPathResultNode<T>) -> StreamCleanNode<T>;
    // Intersections are sorted along the clip edge. For both antimeridian cutting
    // and circle clipPIng, the same comparison is used.
    fn compare_intersection(&self, a: Ci<T>, b: Ci<T>) -> T {
        let a_dashed = a.x;
        let part1 = match a_dashed.x < T::zero() {
            true => a_dashed.y - T::FRAC_PI_2() - T::epsilon(),
            false => T::FRAC_PI_2() - a_dashed.y,
        };
        let b_dashed = b.x;
        let part2 = match b_dashed.x < T::zero() {
            true => b_dashed.y - T::FRAC_PI_2() - T::epsilon(),
            false => T::FRAC_PI_2() - b_dashed.y,
        };

        return part1 - part2;
    }
}

/// Node - holds state associated with the input/output of a StreamProcessor.
/// Something that can be cloned and mutated.
pub type StreamSimpleNode<T> = Rc<RefCell<Box<dyn StreamSimple<T>>>>;
impl<T> Stream<T> for StreamSimpleNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamSimpleNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamSimpleNode<T> where T: CoordFloat + FloatConst {}

pub type StreamPathResultNode<T> = Rc<RefCell<Box<dyn StreamPathResult<T>>>>;
impl<T> Stream<T> for StreamPathResultNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamPathResultNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamPathResultNode<T> where T: CoordFloat + FloatConst {}

pub trait StreamPreClipTrait<T>: StreamClipTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_resample_in(&mut self, _stream: StreamResampleNode<T>) {
        // self.stream = stream;
    }
}
pub struct StreamPreClipNodeStub {}
impl StreamPreClipNodeStub {
    #[inline]
    pub fn new<T>() -> StreamPreClipNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(Self {})))
    }
}

impl<T> StreamPreClipTrait<T> for StreamPreClipNodeStub
where
    T: CoordFloat + FloatConst,
{
    fn stream_resample_in(&mut self, stream: StreamResampleNode<T>) {
        // No-op
    }
}
impl<T> Stream<T> for StreamPreClipNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamClipTrait<T> for StreamPreClipNodeStub where T: CoordFloat + FloatConst {}

pub trait StreamPostClipTrait<T>: StreamClipTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, stream: StreamSimpleNode<T>) {
        // self.stream = stream;
    }
}
pub struct StreamPostClipNodeStub {}
impl StreamPostClipNodeStub {
    #[inline]
    pub fn new<T>() -> StreamPostClipNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(Self {})))
    }
}

impl<T> StreamPostClipTrait<T> for StreamPostClipNodeStub where T: CoordFloat + FloatConst {}

impl<T> Stream<T> for StreamPostClipNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamClipTrait<T> for StreamPostClipNodeStub where T: CoordFloat + FloatConst {}

pub type StreamPreClipNode<T> = Rc<RefCell<Box<dyn StreamPreClipTrait<T>>>>;
impl<T> StreamClipTrait<T> for StreamPreClipNode<T> where T: CoordFloat + FloatConst {}
impl<T> Stream<T> for StreamPreClipNode<T> where T: CoordFloat + FloatConst {}

impl<T> StreamPreClipTrait<T> for StreamPreClipNode<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn stream_resample_in(&mut self, _stream: StreamResampleNode<T>) {
        // No-op.
    }
}

pub type StreamPostClipNode<T> = Rc<RefCell<Box<dyn StreamPostClipTrait<T>>>>;
impl<T> StreamPostClipTrait<T> for StreamPostClipNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamClipTrait<T> for StreamPostClipNode<T> where T: CoordFloat + FloatConst {}
impl<T> Stream<T> for StreamPostClipNode<T> where T: CoordFloat + FloatConst {}

use crate::projection::stream_transform::StreamPreclipIn;
pub type StreamTransformNode<T> = Rc<RefCell<Box<StreamTransform<T>>>>;
impl<T> Stream<T> for StreamTransformNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamTransformNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamPreclipIn<T> for StreamTransformNode<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn stream_preclip_in(&mut self, _stream: StreamPreClipNode<T>) {
        // No-op
    }
}

pub struct StreamSimpleNodeStub {}
impl StreamSimpleNodeStub {
    #[inline]
    pub fn new<T>() -> StreamSimpleNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(Self {})))
    }
}
impl<T> Stream<T> for StreamSimpleNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamSimpleNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamSimpleNodeStub where T: CoordFloat + FloatConst {}

pub struct StreamTransformNodeStub {}
impl StreamTransformNodeStub {
    #[inline]
    pub fn new<T>() -> StreamTransformNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(StreamTransform {
            transform: Rc::new(Box::new(TransformIdentity {})),
            stream: StreamPreClipNodeStub::new(),
        })))
    }
}

impl<T> Stream<T> for StreamTransformNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamTransformNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamTransformNodeStub where T: CoordFloat + FloatConst {}

pub struct StreamNodeStub {}
impl StreamNodeStub {
    #[inline]
    pub fn new<T>() -> StreamSimpleNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(StreamIdentity {})))
    }
}
impl<T> Stream<T> for StreamNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamNodeStub where T: CoordFloat + FloatConst {}

pub struct StreamPathResultNodeStub {}
impl StreamPathResultNodeStub {
    #[inline]
    pub fn new<T>() -> StreamPathResultNode<T>
    where
        T: CoordFloat + FloatConst,
    {
        Rc::new(RefCell::new(Box::new(StreamPathResultIdentity {})))
    }
}
impl<T> Stream<T> for StreamPathResultNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamPathResultNodeStub where T: CoordFloat + FloatConst {}
impl<T> StreamInTrait<T> for StreamPathResultNodeStub where T: CoordFloat + FloatConst {}

pub struct StreamClipIdentity {}
impl<T> Stream<T> for StreamClipIdentity where T: CoordFloat + FloatConst {}
impl<T> StreamClipTrait<T> for StreamClipIdentity where T: CoordFloat + FloatConst {}
impl<T> StreamSimple<T> for StreamClipIdentity where T: CoordFloat + FloatConst {}

impl<T> StreamInTrait<T> for StreamClipIdentity where T: CoordFloat + FloatConst {}
impl<T> BufferInTrait<T> for StreamClipIdentity where T: CoordFloat + FloatConst {}
