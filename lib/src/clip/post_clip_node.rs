use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::rectangle::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

/// A Stream pipeline stage.
#[derive(Clone, Debug)]
pub enum PostClipNode<EP, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    /// Default pass thru.
    I(StreamNode<EP, Identity, SINK, T>),
    /// Clipping rectangle.
    R(StreamNode<EP, Rectangle<T>, SINK, T>),
}

impl<'a, EP, SINK, T> Stream for PostClipNode<EP, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        match self {
            PostClipNode::I(i) => i.get_endpoint(),
            PostClipNode::R(r) => r.get_endpoint(),
        }
    }

    fn sphere(&mut self) {
        match self {
            PostClipNode::I(i) => i.sphere(),
            PostClipNode::R(r) => r.sphere(),
        };
    }
    fn polygon_start(&mut self) {
        match self {
            PostClipNode::I(i) => i.polygon_start(),
            PostClipNode::R(r) => r.polygon_start(),
        };
    }
    fn polygon_end(&mut self) {
        match self {
            PostClipNode::I(i) => i.polygon_end(),
            PostClipNode::R(r) => r.polygon_end(),
        };
    }
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self {
            PostClipNode::I(i) => i.point(p, m),
            PostClipNode::R(r) => r.point(p, m),
        };
    }
    fn line_start(&mut self) {
        match self {
            PostClipNode::I(i) => i.line_start(),
            PostClipNode::R(r) => r.line_start(),
        };
    }
    fn line_end(&mut self) {
        match self {
            PostClipNode::I(i) => i.line_end(),
            PostClipNode::R(r) => r.line_end(),
        };
    }
}
