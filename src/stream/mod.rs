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

pub mod stream_identity;
pub mod stream_path_result_identity;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::AddAssign;
// use crate::clip::BufferInTrait;
// use crate::clip::ClipTraitRaw;
use crate::centroid::centroid_stream::CentroidStream;
use crate::circle::circle::CircleStream;
use crate::length::LengthStream;
use crate::path::area_stream::PathAreaStream;
use crate::path::PathResult;
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

#[derive(Clone, Debug)]
pub enum StreamDst<T>
where
    T: CoordFloat + Default + FloatConst,
{
    SRC(StreamSourceDummy<T>),
    PAS(PathAreaStream<T>),
    CS(CentroidStream<T>),
    LS(LengthStream<T>),
    Circle(CircleStream<T>),
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

impl<T> Stream<T> for StreamDst<T>
where
    T: CoordFloat + Debug + Default + FloatConst + AddAssign,
{
    type C = Coordinate<T>;

    fn get_dst(&self) -> StreamDst<T> {
        match self {
            // StreamDst::SRC(src) => src.get_dst(),
            StreamDst::PAS(pas) => pas.get_dst(),
            StreamDst::CS(cs) => cs.get_dst(),
            StreamDst::LS(ls) => ls.get_dst(),
            StreamDst::Circle(c) => c.get_dst(),
            StreamDst::SRC(_src) => {
                todo!("handle dummy")
            }
        }
    }
    fn sphere(&mut self) {
        match self {
            StreamDst::PAS(pas) => pas.sphere(),
            StreamDst::CS(cs) => cs.sphere(),
            StreamDst::LS(ls) => ls.sphere(),
            StreamDst::Circle(c) => c.sphere(),
            StreamDst::SRC(_src) => {
                todo!("handle dummy")
            }
        }
    }
    fn polygon_start(&mut self) {
        match self {
            StreamDst::PAS(pas) => pas.polygon_start(),
            StreamDst::CS(cs) => cs.polygon_start(),
            StreamDst::LS(ls) => ls.polygon_start(),
            StreamDst::Circle(c) => c.polygon_start(),
            StreamDst::SRC(_src) => {
                todo!("handle dummy")
            }
        }
    }
    fn polygon_end(&mut self) {
        match self {
            StreamDst::PAS(pas) => pas.polygon_end(),
            StreamDst::CS(cs) => cs.polygon_end(),
            StreamDst::LS(ls) => ls.polygon_end(),
            StreamDst::Circle(c) => c.polygon_end(),
            StreamDst::SRC(_src) => {
                todo!("handle dummy")
            }
        }
    }
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self {
            StreamDst::PAS(pas) => pas.point(p, m),
            StreamDst::CS(cs) => cs.point(p, m),
            StreamDst::LS(ls) => ls.point(p, m),
            StreamDst::Circle(c) => c.point(p, m),
            StreamDst::SRC(_src) => {
                todo!("handle dummy")
            }
        }
    }
    fn line_start(&mut self) {
        match self {
            StreamDst::PAS(pas) => pas.line_start(),
            StreamDst::CS(cs) => cs.line_start(),
            StreamDst::LS(ls) => ls.line_start(),
            StreamDst::Circle(c) => c.line_start(),
            StreamDst::SRC(_src) => {
                todo!("handle dummy")
            }
        }
    }
    fn line_end(&mut self) {
        match self {
            StreamDst::PAS(pas) => pas.line_end(),
            StreamDst::CS(cs) => cs.line_end(),
            StreamDst::LS(ls) => ls.line_end(),
            StreamDst::Circle(c) => c.line_end(),
            StreamDst::SRC(_src) => {
                todo!("handle dummy")
            }
        }
    }
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
