use crate::projection::NodeFactory;
use crate::projection::StreamNode;
use core::marker::PhantomData;
use geo::CoordFloat;
use geo::Coordinate;
use std::cell::RefCell;
use std::rc::Rc;

// use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use std::fmt::Display;
use std::ops::AddAssign;

use crate::stream::Stream;

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
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    phantomT: PhantomData<T>,
    phantomSINK: PhantomData<SINK>,
    pub raw: RAW,
}

impl<RAW, SINK, T> StreamNodeFactory<RAW, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(raw: RAW) -> StreamNodeFactory<RAW, SINK, T> {
        StreamNodeFactory {
            phantomT: PhantomData::<T>,
            phantomSINK: PhantomData::<SINK>,
            raw,
        }
    }
}

impl<RAW, SINK, T> NodeFactory for StreamNodeFactory<RAW, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type Sink = SINK;
    type Raw = RAW;
    type T = T;
    fn generate(
        &self,
        sink: Rc<RefCell<Self::Sink>>,
    ) -> Rc<RefCell<StreamNode<Self::Raw, Self::Sink, Self::T>>> {
        Rc::new(RefCell::new(StreamNode {
            raw: self.raw,
            sink,
            pd: PhantomData::<T>,
        }))
    }
}
