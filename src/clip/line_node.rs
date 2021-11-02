use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use super::line::Line;
use super::{Clean, CleanState};
use crate::clip::LineAntimeridian;
use crate::clip::LineCircle;
use crate::{projection::stream_node::StreamNode, stream::Stream};

#[derive(Clone, Debug)]
pub enum LineNode<SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    C(StreamNode<LineCircle<T>, SINK, T>),
    A(StreamNode<LineAntimeridian<T>, SINK, T>),
}

impl<T> Clean for Line<T>
where
    T: CoordFloat + FloatConst,
{
    fn clean(&self) -> CleanState {
        match self {
            Line::C(l) => l.clean(),
            Line::A(l) => l.clean(),
        }
    }
}

impl<SINK, T> Stream for LineNode<SINK, T>
where
    SINK: Stream<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
        match self {
            LineNode::C(l) => l.point(p, m),
            LineNode::A(l) => l.point(p, m),
        }
    }

    #[inline]
    fn sphere(&mut self) {
        match self {
            LineNode::C(l) => l.sphere(),
            LineNode::A(l) => l.sphere(),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self {
            LineNode::C(l) => l.line_start(),
            LineNode::A(l) => l.line_start(),
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self {
            LineNode::C(l) => l.line_end(),
            LineNode::A(l) => l.line_end(),
        }
    }

    #[inline]
    fn polygon_start(&mut self) {
        match self {
            LineNode::C(l) => l.polygon_start(),
            LineNode::A(l) => l.polygon_start(),
        }
    }

    #[inline]
    fn polygon_end(&mut self) {
        match self {
            LineNode::C(l) => l.polygon_end(),
            LineNode::A(l) => l.polygon_end(),
        }
    }
}
