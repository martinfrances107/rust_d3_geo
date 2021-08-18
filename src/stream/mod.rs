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

use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

/// Applies to DataObject's
pub trait Streamable {
    type T: CoordFloat;
    fn to_stream<SD: Stream<T = Self::T>>(&self, stream: &mut SD);
}

#[derive(Clone, Debug)]
pub struct StreamDrainStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Stream for StreamDrainStub<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn point(&mut self, _p: &Coordinate<T>, _m: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}

impl<T> Default for StreamDrainStub<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

// #[derive(Clone, Debug)]
// pub struct StreamStageStub<STREAMIN, T>
// where
//     T: CoordFloat,
// {
//     phantom: PhantomData<T>,
//     phantomInput: PhantomData<STREAMIN>,
// }

// impl<STREAMIN, T> Default for StreamStageStub<STREAMIN, T>
// where
//     T: CoordFloat,
// {
//     fn default() -> Self {
//         Self {
//             phantom: PhantomData::default(),
//             phantomInput: PhantomData::default(),
//         }
//     }
// }

// impl<STREAMIN, T> Stream for StreamStageStub<STREAMIN, T>
// where
//     T: CoordFloat,
// {
//     type T=T;
//     fn point(&mut self, _p: &Self::SC, _m: Option<u8>) {}
//     fn sphere(&mut self) {}
//     fn line_start(&mut self) {}
//     fn line_end(&mut self) {}
//     fn polygon_start(&mut self) {}
//     fn polygon_end(&mut self) {}
// }

/// Default implmentation is a no-op.
pub trait Stream: Clone
where
    <Self as Stream>::T: CoordFloat,
{
    type T;

    fn point(&mut self, _p: &Coordinate<Self::T>, _m: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}

/// Ci CompareIntersections param type
/// See StreamClip.
#[derive(Clone, Debug, Default)]
pub struct CompareIntersection<T: CoordFloat>
where
    T: CoordFloat,
{
    x: Coordinate<T>,
}
