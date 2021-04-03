use std::fmt::Debug;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::centroid::centroid_stream::CentroidStream;
use crate::circle::circle::CircleStream;
use crate::length::LengthStream;
use crate::path::area_stream::PathAreaStream;

use super::Stream;
use super::StreamSourceDummy;

#[derive(Clone, Debug)]
pub enum StreamDst<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    SRC(StreamSourceDummy<T>),
    PAS(PathAreaStream<T>),
    CS(CentroidStream<T>),
    LS(LengthStream<T>),
    Circle(CircleStream<T>),
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
