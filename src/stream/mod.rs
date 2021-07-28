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
mod rect;
mod stream_line;
mod triangle;

pub mod stream_dst;
pub mod stream_in_trait;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use stream_dst::StreamDst;

/// Applies to DataObject's
pub trait Streamable {
    type T: AddAssign + AsPrimitive<Self::T> + CoordFloat + Display + FloatConst;
    fn to_stream<SD: Stream<SC = Coordinate<Self::T>>>(&self, stream: &mut SD);
}

#[derive(Clone, Debug)]
pub struct StreamDummy<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Default for StreamDummy<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

impl<T> Stream for StreamDummy<T>
where
    T: CoordFloat,
{
    type SC = Coordinate<T>;
    fn point(&mut self, _p: &Self::SC, _m: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}

pub trait Stream {
    type SC;

    fn point(&mut self, _p: &Self::SC, _m: Option<u8>);
    fn sphere(&mut self);
    fn line_start(&mut self);
    fn line_end(&mut self);
    fn polygon_start(&mut self);
    fn polygon_end(&mut self);
}

/// Ci CompareIntersections param type
/// See StreamClip.
#[derive(Clone, Debug, Default)]
pub struct CompareIntersection<T: CoordFloat>
where
    T: CoordFloat + FloatConst,
{
    x: Coordinate<T>,
}
