use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::rectangle::rectangle::Rectangle;
use crate::identity::Identity;
use crate::stream::Stream;

use super::resample::resample::Resample;
use super::stream_node::StreamNode;
use super::stream_node_factory::StreamNodeFactory;
use super::{NodeFactory, Raw};

#[derive(Clone, Debug)]
pub(crate) enum PostClipType<PR, T>
where
    PR: Raw<T>,
    T: CoordFloat,
{
    I(Identity),
    R(Rectangle<PR, T>),
}

impl<PR, SINK, T> NodeFactory for StreamNodeFactory<PostClipType<PR, T>, SINK, T>
where
    PR: Raw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type Sink = SINK;
    type T = T;
    type Node = StreamNode<Resample<PR, T>, Self::Sink, Self::T>;
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> Self::Node {}
}
