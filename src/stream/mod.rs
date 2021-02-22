pub mod feature_collection;
pub mod geometry;
pub mod geometry_collection;
pub mod line;
pub mod line_string;
pub mod multi_line_string;
pub mod multi_point;
pub mod multi_polygon;
pub mod point;
pub mod polygon;

mod geometry_processor;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::BufferInTrait;
use crate::path::PathResult;
use crate::projection::stream_transform::StreamPreclipIn;
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
    stream: StreamSimpleNode<T>,
}

impl<T> Default for StreamIdentity<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn default() -> Self {
        Self {
            stream: Rc::new(RefCell::new(StreamDummy::default())),
        }
    }
}
impl<T> Stream for StreamIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let mut s = self.stream.borrow_mut();
        s.point(p, m);
    }
    #[inline]
    fn sphere(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.sphere();
    }
    #[inline]
    fn line_start(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.line_start();
    }
    #[inline]
    fn line_end(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.line_end();
    }
    #[inline]
    fn polygon_start(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.polygon_start();
    }
    #[inline]
    fn polygon_end(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.polygon_end();
    }
}
impl<T> StreamInTrait<T> for StreamIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, stream: StreamSimpleNode<T>) {
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
impl<T> StreamPathResult<T> for StreamPathResultIdentity<T> where T: CoordFloat + FloatConst {}
impl<T> Stream for StreamPathResultIdentity<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamPathResultIdentity<T> where T: CoordFloat + FloatConst {}
impl<T> PathResult<T> for StreamPathResultIdentity<T> where T: CoordFloat + FloatConst {}

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
    fn clean(&self) -> CleanEnum;
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

pub trait StreamInTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, _stream: StreamSimpleNode<T>) {}
}

pub trait StreamClipLine<T>: Stream + Clean + BufferInTrait<T>
where
    T: CoordFloat + FloatConst,
{
}

pub trait StreamClean<T>: Stream + Clean
where
    T: CoordFloat + FloatConst,
{
}

pub trait StreamPathResult<T>: Stream + PathResult<T>
where
    T: CoordFloat + FloatConst,
{
}

pub type StreamClipLineNode<T> = Rc<RefCell<dyn StreamClipLine<T, C = Coordinate<T>>>>;
impl<T> Stream for StreamClipLineNode<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}
impl<T> BufferInTrait<T> for StreamClipLineNode<T> where T: CoordFloat + FloatConst {}

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

impl<T> StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    pub fn new() -> StreamClipLineNode<T> {
        Rc::new(RefCell::new(Self::default()))
    }
}
impl<T> StreamClipLine<T> for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst {}
impl<T> Stream for StreamClipLineNodeStub<T>
where
    T: CoordFloat + FloatConst,
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
impl<T> BufferInTrait<T> for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst {}
impl<T> StreamPreClipTrait<T> for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst {}

impl<T> StreamClipTrait<T> for StreamClipLineNodeStub<T> where T: CoordFloat + FloatConst {}

pub trait StreamResampleTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_postclip_in(&mut self, stream_clip_in: StreamPostClipNode<T>);
}
pub type StreamResampleNode<T> = Rc<RefCell<dyn StreamResampleTrait<T>>>;
impl<T> StreamResampleTrait<T> for StreamResampleNode<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_postclip_in(&mut self, _stream_clip_in: StreamPostClipNode<T>) {
        // No-op.
    }
}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamResampleNodeStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> StreamResampleNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn new() -> StreamResampleNode<T> {
        Rc::new(RefCell::new(Self::default()))
    }
}
impl<T> StreamResampleTrait<T> for StreamResampleNodeStub<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_postclip_in(&mut self, _stream: StreamPostClipNode<T>) {
        // No-op.
    }
}
impl<T> Stream for StreamResampleNodeStub<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}

/// Ci CompareIntersections param type
/// See StreamClipTrait.
#[derive(Clone, Debug, Default)]
pub struct CompareIntersection<T: CoordFloat>
where
    T: CoordFloat + FloatConst,
{
    x: Coordinate<T>,
}
pub trait StreamClipTrait<T>: Stream
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
        panic!("Must override.");
    }

    #[inline]
    fn point_visible(&self, _p: Coordinate<T>, _z: Option<u8>) -> bool {
        panic!("Must override.");
    }

    // fn clip_line(&self, stream: StreamPathResultNode<T>) -> StreamCleanNode<T>;
    // Intersections are sorted along the clip edge. For both antimeridian cutting
    // and circle clipPIng, the same comparison is used.
    fn compare_intersection(&self, a: CompareIntersection<T>, b: CompareIntersection<T>) -> T {
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

pub type StreamSimpleNode<T> = Rc<RefCell<dyn Stream<C = Coordinate<T>>>>;
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

pub type StreamPathResultNode<T> = Rc<RefCell<dyn StreamPathResult<T, C = Coordinate<T>>>>;
impl<T> Stream for StreamPathResultNode<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamPathResultNode<T> where T: CoordFloat + FloatConst {}

pub trait StreamPreClipTrait<T>: StreamClipTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_resample_in(&mut self, _stream: StreamResampleNode<T>) {
        // No-op.
    }
}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamPreClipNodeStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}
impl<T> StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    pub fn new() -> StreamPreClipNode<T> {
        Rc::new(RefCell::new(Self::default()))
    }
}

impl<T> StreamPreClipTrait<T> for StreamPreClipNodeStub<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_resample_in(&mut self, _stream: StreamResampleNode<T>) {
        // No-op.
    }
}
impl<T> Stream for StreamPreClipNodeStub<T>
where
    T: CoordFloat,
{
    type C = Coordinate<T>;
}
impl<T> StreamClipTrait<T> for StreamPreClipNodeStub<T> where T: CoordFloat + FloatConst {}

pub trait StreamPostClipTrait<T>: StreamClipTrait<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_in(&mut self, _stream: StreamSimpleNode<T>) {
        // No-op.
    }
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
impl<T> StreamPostClipNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    pub fn new() -> StreamPostClipNode<T> {
        Rc::new(RefCell::new(Self::default()))
    }
}

impl<T> StreamPostClipTrait<T> for StreamPostClipNodeStub<T> where T: CoordFloat + FloatConst {}

impl<T> Stream for StreamPostClipNodeStub<T>
where
    T: CoordFloat,
{
    type C = Coordinate<T>;
}
impl<T> StreamClipTrait<T> for StreamPostClipNodeStub<T> where T: CoordFloat + FloatConst {}

pub type StreamPreClipNode<T> = Rc<RefCell<dyn StreamPreClipTrait<T, C = Coordinate<T>>>>;
impl<T> StreamClipTrait<T> for StreamPreClipNode<T> where T: CoordFloat + FloatConst {}
impl<T> Stream for StreamPreClipNode<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}

impl<T> StreamPreClipTrait<T> for StreamPreClipNode<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn stream_resample_in(&mut self, _stream: StreamResampleNode<T>) {
        // No-op.
    }
}

pub type StreamPostClipNode<T> = Rc<RefCell<dyn StreamPostClipTrait<T, C = Coordinate<T>>>>;
impl<T> StreamPostClipTrait<T> for StreamPostClipNode<T> where T: CoordFloat + FloatConst {}
impl<T> StreamClipTrait<T> for StreamPostClipNode<T> where T: CoordFloat + FloatConst {}
impl<T> Stream for StreamPostClipNode<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}

pub type StreamTransformNode<T> = Rc<RefCell<StreamTransform<T>>>;
impl<T> Stream for StreamTransformNode<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}
impl<T> StreamPreclipIn<T> for StreamTransformNode<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn stream_preclip_in(&mut self, _stream: StreamPreClipNode<T>) {
        // No-op.
    }
}

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

impl<T> Stream for StreamDummy<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamDummy<T> where T: CoordFloat + FloatConst {}

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamTransformNodeStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> StreamTransformNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    pub fn new() -> StreamTransformNode<T> {
        Rc::new(RefCell::new(StreamTransform {
            transform: Rc::new(Box::new(TransformIdentity::default())),
            stream: StreamPreClipNodeStub::new(),
        }))
    }
}

impl<T> Stream for StreamTransformNodeStub<T>
where
    T: CoordFloat + FloatConst,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamTransformNodeStub<T> where T: CoordFloat + FloatConst {}

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
        Rc::new(RefCell::new(StreamNodeStub::<T>::default()))
    }
}
impl<T> Stream for StreamNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
}
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

impl<T> StreamPathResultNodeStub<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    pub fn new() -> StreamPathResultNode<T> {
        Rc::new(RefCell::new(StreamPathResultIdentity::default()))
    }
}
impl<T> Stream for StreamPathResultNodeStub<T>
where
    T: CoordFloat,
{
    type C = Coordinate<T>;
}
impl<T> StreamInTrait<T> for StreamPathResultNodeStub<T> where T: CoordFloat + FloatConst {}
