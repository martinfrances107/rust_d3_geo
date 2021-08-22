use crate::projection::resample::Resample;
use std::cell::RefCell;
use std::rc::Rc;

use core::marker::PhantomData;
use derivative::Derivative;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::stream_node::StreamNode;
use crate::projection::NodeFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use super::none::None as ResampleNone;
use super::ResampleNode;

/// Used in the construct of a Projection stream pipeline.
///
/// Stream Raw (SR) is the proto-node. ( The node without the link to other node's up the chain)
///
/// SR is precomputed and held in the projection.
///
/// Inside Projection::stream() NodeFactory::generate() will be called to
/// construct the pipeline.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct StreamNodeResampleFactory<PR, SINK, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat,
{
    phantom_sink: PhantomData<SINK>,
    projection_raw: PR,
    delta2: T,
}

impl<PR, SINK, T> StreamNodeResampleFactory<PR, SINK, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat,
{
    pub fn new(projection_raw: PR, delta2: T) -> StreamNodeResampleFactory<PR, SINK, T> {
        // let interpolate_factory = StreamNodeFactory::new(interpolate_raw);
        StreamNodeResampleFactory {
            delta2,
            projection_raw,
            phantom_sink: PhantomData::<SINK>,
            // projection,
        }
    }
}

impl<PR, SINK, T> NodeFactory for StreamNodeResampleFactory<PR, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type Sink = SINK;
    type T = T;
    type Raw = ResampleNode<PR, SINK, T>;
    type Node = ResampleNode<PR, SINK, Self::T>;
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> Self::Node {
        match self.delta2.is_zero() {
            true => ResampleNode::RN(StreamNode {
                raw: ResampleNone::new(PR::default()),
                sink,
                pd: PhantomData::<T>,
            }),
            false => ResampleNode::R(StreamNode {
                raw: Resample::new(self.projection_raw, self.delta2),
                sink,
                pd: PhantomData::<T>,
            }),
        }
        // StreamNodeFactory::new(resample).generate(sink)
    }
}
