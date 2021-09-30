use std::cell::RefCell;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::rectangle::rectangle::Rectangle;
use crate::identity::Identity;
use crate::stream::Stream;

use super::resample::resample::Resample;
use super::stream_node::StreamNode;
use super::stream_node_factory::StreamNodeFactory;
use super::{NodeFactory, Raw};

#[derive(Clone, Debug)]
pub enum PostClipNode<SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    I(StreamNode<Identity, SINK, T>),
    R(StreamNode<Rectangle<T>, SINK, T>),
}

impl<'a, SINK, T> Stream for PostClipNode<SINK, T>
where
    SINK: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

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
