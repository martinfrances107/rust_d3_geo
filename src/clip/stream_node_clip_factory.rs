use crate::clip::buffer::Buffer;
use crate::clip::clip::Clip;
use crate::clip::line_elem::LineElem;
use crate::clip::InterpolateRaw;
use crate::clip::LineRaw;
use crate::projection::stream_node::StreamNode;

use crate::clip::PointVisible;
use std::collections::VecDeque;
// use crate::clip::Rejoin;
// use crate::projection::stream_node::StreamNode;
use crate::projection::stream_node_factory::StreamNodeFactory;
use crate::projection::NodeFactory;
// use crate::Transform;
use core::marker::PhantomData;
use geo::CoordFloat;
use geo::Coordinate;
// use num_traits::Float;
// use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

// use crate::clip::antimeridian::interpolate::Interpolate as InterpolateClipAntimeridian;
// use crate::clip::antimeridian::line::Line as LineClipAntimeridian;
// use crate::clip::antimeridian::pv::PV as PonitVisibleClipAntimeridian;
use crate::stream::Stream;

/// Used in the construct of a Projection stream pipeline.
///
/// Stream Raw (SR) is the proto-node. ( The node without the link to other node's up the chain)
///
/// SR is precomputed and held in the projection.
///
/// Inside Projection::stream() NodeFactory::generate() will be called to
/// construct the pipeline.
#[derive(Clone, Debug)]
pub struct StreamNodeClipFactory<I, L, PV, SINK, T>
where
    I: InterpolateRaw,
    L: LineRaw,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    phantomT: PhantomData<T>,
    pv: PV,
    line_ring_buffer_factory: StreamNodeFactory<L, Buffer<T>, T>,
    line_sink_factory: StreamNodeFactory<L, SINK, T>,
    interpolate_factory: StreamNodeFactory<I, SINK, T>,
}

impl<I, L, PV, SINK, T> StreamNodeClipFactory<I, L, PV, SINK, T>
where
    I: InterpolateRaw,
    L: LineRaw,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(
        interpolate_raw: I,
        line_raw: L,
        pv: PV,
    ) -> StreamNodeClipFactory<I, L, PV, SINK, T> {
        let line_ring_buffer_factory: StreamNodeFactory<L, Buffer<T>, T> =
            StreamNodeFactory::new(line_raw);
        let line_sink_factory = StreamNodeFactory::new(line_raw);
        let interpolate_factory = StreamNodeFactory::new(interpolate_raw);
        StreamNodeClipFactory {
            interpolate_factory,
            line_ring_buffer_factory,
            line_sink_factory,
            phantomT: PhantomData::<T>,
            pv,
        }
    }
}

impl<I, L, PV, SINK, T> NodeFactory for StreamNodeClipFactory<I, L, PV, SINK, T>
where
    I: InterpolateRaw,
    L: LineRaw,
    PV: PointVisible,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type Sink = SINK;
    type Raw = Clip<I, L, PV, SINK, T>;
    type T = T;
    fn generate(
        &self,
        sink: Rc<RefCell<Self::Sink>>,
    ) -> Rc<RefCell<StreamNode<Self::Raw, Self::Sink, Self::T>>>
// where
    //     PR: Transform<C = Coordinate<T>>,
    {
        let start = LineElem {
            p: Coordinate {
                x: -T::PI(),
                y: -T::PI() / T::from(2_u8).unwrap(),
            },
            m: None,
        };

        let ring_buffer_node: Rc<RefCell<Buffer<T>>> = Rc::new(RefCell::new(Buffer::default()));
        let mut ring_sink = self.line_ring_buffer_factory.generate(ring_buffer_node);
        Rc::new(RefCell::new(StreamNode {
            raw: Clip {
                pv: self.pv,
                line_node: self.line_sink_factory.generate(sink),
                interpolate_node: self.interpolate_factory.generate(sink),
                start,

                polygon_started: false,
                polygon: Vec::new(),
                ring: Vec::new(),
                // ring_sink,
                ring_buffer_node,
                segments: VecDeque::new(),

                use_point_line: false,
                use_ring_start: false,
                use_ring_end: false,
            },
            sink,
            pd: PhantomData::<T>,
        }))
    }
}
