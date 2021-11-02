use core::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::stream_node::StreamNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::NodeFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use super::buffer::Buffer;
use super::clip::Clip;
use super::line::Line;
use super::line_elem::LineElem;
use super::line_node::LineNode;
use super::stream_node_line_factory::StreamNodeLineFactory;
use super::InterpolateFn;
use super::PointVisible;

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
pub struct StreamNodeClipFactory<PR, PV, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    phantom_pr: PhantomData<PR>,

    // Passed to ::generate()
    pv: PV,
    #[derivative(Debug = "ignore")]
    interpolate_fn: InterpolateFn<SINK, T>,
    line_node_factory: StreamNodeLineFactory<SINK, T>,

    // Precomputed pair.
    ring_buffer: Rc<RefCell<Buffer<T>>>,
    ring_sink_node: LineNode<Buffer<T>, T>,
}

impl<PR, PV, SINK, T> StreamNodeClipFactory<PR, PV, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    pub fn new(
        interpolate_fn: InterpolateFn<SINK, T>,
        line: Line<T>,
        pv: PV,
    ) -> StreamNodeClipFactory<PR, PV, SINK, T> {
        let line_node_factory = StreamNodeLineFactory::new(line.clone());

        // ring_buffer needs the Rc<RefCell<>> wrapper because it is a pipeline source
        // [internal to the clip node].
        let ring_buffer: Rc<RefCell<Buffer<T>>> = Rc::new(RefCell::new(Buffer::default()));
        let line_node_buffer_factory = StreamNodeLineFactory::new(line);
        let ring_sink_node = line_node_buffer_factory.generate(ring_buffer.clone());

        StreamNodeClipFactory {
            ring_buffer,
            ring_sink_node,
            interpolate_fn,
            line_node_factory,
            phantom_pr: PhantomData::<PR>,
            pv,
        }
    }
}

impl<PR, PV, SINK, T> NodeFactory for StreamNodeClipFactory<PR, PV, SINK, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type Sink = SINK;
    type T = T;

    type Node = StreamNode<Clip<PV, SINK, T>, Self::Sink, Self::T>;
    fn generate(&self, sink: Rc<RefCell<Self::Sink>>) -> Self::Node {
        let start = LineElem {
            p: Coordinate {
                x: -T::PI(),
                y: -T::PI() / T::from(2_u8).unwrap(),
            },
            m: None,
        };

        let clip = Clip::new(
            self.pv.clone(),
            self.line_node_factory.clone(),
            self.interpolate_fn.clone(),
            self.ring_buffer.clone(),
            self.ring_sink_node.clone(),
            sink.clone(),
            start,
        );
        StreamNodeFactory::new(clip).generate(sink)
    }
}
