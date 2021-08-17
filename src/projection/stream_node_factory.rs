use core::marker::PhantomData;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

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
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    phantom_t: PhantomData<T>,
    phantom_sink: PhantomData<SINK>,
    pub raw: RAW,
}

impl<RAW, SINK, T> StreamNodeFactory<RAW, SINK, T>
where
    SINK: Stream<T = T>,
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(raw: RAW) -> StreamNodeFactory<RAW, SINK, T> {
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
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type Sink = SINK;
    type Raw = RAW;
    type T = T;
    fn generate(
        &self,
        sink: Rc<RefCell<Self::Sink>>,
    ) -> StreamNode<Self::Raw, Self::Sink, Self::T> {
        StreamNode {
            raw: self.raw.clone(),
            sink,
            pd: PhantomData::<T>,
        }
    }
}
