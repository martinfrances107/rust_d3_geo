use super::buffer::Buffer;
use super::clip::Clip;
use super::line_elem::LineElem;
use super::InterpolateFn;
use super::Line;
use super::PointVisible;

use std::cell::RefCell;
use std::rc::Rc;

use core::marker::PhantomData;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::stream_node::StreamNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::NodeFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use derivative::Derivative;

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
pub struct StreamNodeClipFactory<L, PR, PV, SINK, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    phantom_pr: PhantomData<PR>,

    // Passed to ::generate()
    pv: PV,
    #[derivative(Debug = "ignore")]
    interpolate_fn: InterpolateFn<SINK, T>,
    line_raw: L,

    // Precomputed pair.
    ring_buffer: Rc<RefCell<Buffer<T>>>,
    ring_sink_node: StreamNode<L, Buffer<T>, T>,
}

impl<L, PR, PV, SINK, T> StreamNodeClipFactory<L, PR, PV, SINK, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    pub fn new(
        interpolate_fn: InterpolateFn<SINK, T>,
        line_raw: L,
        pv: PV,
    ) -> StreamNodeClipFactory<L, PR, PV, SINK, T> {
        let line_ring_buffer_factory = StreamNodeFactory::new(line_raw.clone());

        // ring_buffer needs the Rc<RefCell<>> wrapper because it is a pipeline source
        // [internal to the clip node].
        let ring_buffer: Rc<RefCell<Buffer<T>>> = Rc::new(RefCell::new(Buffer::default()));
        let ring_sink_node = line_ring_buffer_factory.generate(ring_buffer.clone());

        // let interpolate_factory = StreamNodeFactory::new(interpolate_raw);
        StreamNodeClipFactory {
            ring_buffer,
            ring_sink_node,

            interpolate_fn,
            // line_ring_buffer,
            line_raw,
            phantom_pr: PhantomData::<PR>,
            pv,
        }
    }
}

impl<L, PR, PV, SINK, T> NodeFactory for StreamNodeClipFactory<L, PR, PV, SINK, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type Sink = SINK;
    type T = T;
    type Raw = Clip<L, PV, SINK, T>;
    fn generate(
        &self,
        sink: Rc<RefCell<Self::Sink>>,
    ) -> StreamNode<Self::Raw, Self::Sink, Self::T> {
        let start = LineElem {
            p: Coordinate {
                x: -T::PI(),
                y: -T::PI() / T::from(2_u8).unwrap(),
            },
            m: None,
        };

        let clip = Clip::new(
            self.pv.clone(),
            self.line_raw.clone(),
            self.interpolate_fn.clone(),
            self.ring_buffer.clone(),
            self.ring_sink_node.clone(),
            sink.clone(),
            start,
        );
        StreamNodeFactory::new(clip).generate(sink)
    }
}
