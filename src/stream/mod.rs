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
mod rect;
mod triangle;

pub mod stream_dst;
pub mod stream_identity;

use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use stream_dst::StreamDst;
/// Applies to DataObject's
pub trait Streamable<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type SC;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>);
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

#[derive(Clone, Default, Debug)]
pub struct StreamSourceDummy<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

pub trait Stream<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type C;
    fn point(&mut self, _p: &Self::C, _m: Option<u8>);
    fn sphere(&mut self);
    fn line_start(&mut self);
    fn line_end(&mut self);
    fn polygon_start(&mut self);
    fn polygon_end(&mut self);
    fn get_dst(&self) -> StreamDst<T>;
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

// Node - holds state associated with the input/output of a StreamProcessor.
// Something that can be cloned and mutated.

pub type StreamSimpleNode<T> = Box<dyn Stream<T, C = Coordinate<T>>>;
