use core::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;

use crate::stream::Stream;

use super::NodeFactory;
use super::StreamNode;

// use super::stream_node::StreamNode;

/// Used in the construct of a Projection stream pipeline.
///
/// RAW is the proto-node. ( The node without the link to other node's up the chain)
///
/// RAW is precomputed and held in the projection.
///
/// Inside Projection::stream() NodeFactory::generate() will be called to
/// construct the pipeline.
#[derive(Copy, Clone, Debug)]
pub struct StreamNodeFactory<RAW, SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    phantom_t: PhantomData<T>,
    phantom_sink: PhantomData<SINK>,
    raw: RAW,
}

impl<RAW, SINK, T> StreamNodeFactory<RAW, SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    pub(crate) fn new(raw: RAW) -> StreamNodeFactory<RAW, SINK, T> {
        StreamNodeFactory {
            phantom_t: PhantomData::<T>,
            phantom_sink: PhantomData::<SINK>,
            raw,
        }
    }
}

/// RAW here can be projection_raw or line_raw,
impl<RAW, SINK, T> NodeFactory for StreamNodeFactory<RAW, SINK, T>
where
    RAW: Clone,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    type Sink = SINK;
    type T = T;
    type Node = StreamNode<RAW, Self::Sink, Self::T>;
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> Self::Node {
        StreamNode {
            raw: self.raw.clone(),
            sink,
            // pd: PhantomData::<T>,
        }
    }
}
