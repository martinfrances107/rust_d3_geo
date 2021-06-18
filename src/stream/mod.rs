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
// use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use stream_dst::StreamDst;

/// Applies to DataObject's
pub trait Streamable {
    type T: AddAssign + AsPrimitive<Self::T> + CoordFloat + Default + Display + FloatConst;
    fn to_stream<SD: Stream<SC = Coordinate<Self::T>>>(&self, stream: &mut SD);
}

// #[derive(Clone, Default, Debug)]
// pub struct StreamSourceDummy<T>
// where
//     T: CoordFloat,
// {
//     phantom: PhantomData<T>,
// }

pub trait Stream {
    type SC;

    fn point(&mut self, _p: &Self::SC, _m: Option<u8>);
    fn sphere(&mut self);
    fn line_start(&mut self);
    fn line_end(&mut self);
    fn polygon_start(&mut self);
    fn polygon_end(&mut self);
    // fn get_dst(
    //     &self,
    // ) -> Box<
    //     dyn StreamDst<
    //         SC = <Self as Stream>::SC,
    //         SD = Self::SD,
    //         T = Self::ST,
    //         ST = Self::ST,
    //         Out = Self::SD,
    //     >,
    // >;
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

// Node - holds state associated with the input/output of a StreamProcessor.
// Something that can be cloned and mutated.

// pub type StreamSimpleNode<T> = Box<dyn Stream>;
