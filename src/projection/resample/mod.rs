pub mod none;
pub mod resample;
pub mod stream_node_resample_factory;

use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::stream_node::StreamNode;
use super::Raw as ProjectionRaw;

use none::None;
use resample::Resample;

#[derive(Debug, Clone)]
pub enum ResampleNode<PR, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    RN(StreamNode<None<PR, T>, SINK, T>),
    R(StreamNode<Resample<PR, T>, SINK, T>),
}

impl<'a, PR, SINK, T> Stream for ResampleNode<PR, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn sphere(&mut self) {
        match self {
            ResampleNode::RN(n) => n.sphere(),
            ResampleNode::R(r) => r.sphere(),
        };
    }
    fn polygon_start(&mut self) {
        match self {
            ResampleNode::RN(n) => n.polygon_start(),
            ResampleNode::R(r) => r.polygon_start(),
        };
    }
    fn polygon_end(&mut self) {
        match self {
            ResampleNode::RN(n) => n.polygon_end(),
            ResampleNode::R(r) => r.polygon_end(),
        };
    }
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self {
            ResampleNode::RN(n) => n.point(p, m),
            ResampleNode::R(r) => r.point(p, m),
        };
    }
    fn line_start(&mut self) {
        match self {
            ResampleNode::RN(n) => n.line_start(),
            ResampleNode::R(r) => r.line_start(),
        };
    }
    fn line_end(&mut self) {
        match self {
            ResampleNode::RN(n) => n.line_end(),
            ResampleNode::R(r) => r.line_end(),
        };
    }
}
