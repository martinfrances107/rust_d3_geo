// use core::marker::PhantomData;
// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use derivative::*;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use crate::clip::Line;
// use crate::projection::stream_node::StreamNode;
// use crate::projection::stream_node_factory::StreamNodeFactory;
// use crate::projection::NodeFactory;
// use crate::projection::ProjectionRawBase;
// use crate::stream::Stream;

// use super::buffer::Buffer;
// use super::clip_node::ClipNode;
// use super::InterpolateFn;
// use super::PointVisible;

// /// Used in the construct of a Projection stream pipeline.
// ///
// /// Stream Raw (SR) is the proto-node. ( The node without the link to other node's up the chain )
// ///
// /// SR is precomputed and held in the projection.
// ///
// /// Inside Projection::stream() NodeFactory::generate() will be called to
// /// construct the pipeline.
// #[derive(Clone, Derivative)]
// #[derivative(Debug)]
// pub struct StreamNodeClipFactory<EP, LINE, PR, PV, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     phantom_pr: PhantomData<PR>,

//     start: Coordinate<T>,
//     pv: PV,
//     #[derivative(Debug = "ignore")]
//     interpolate_fn: InterpolateFn<SINK, T>,
//     line_node_factory: StreamNodeFactory<EP, LINE, SINK, T>,

//     ring_sink_node: StreamNode<Buffer<T>, LINE, Buffer<T>, T>,
// }

// impl<EP, LINE, PR, PV, SINK, T> StreamNodeClipFactory<EP, LINE, PR, PV, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     /// Constructor.
//     pub fn new(
//         pv: PV,
//         LB: Line, LC: Line, LU: Line,
//         interpolate_fn: InterpolateFn<SINK, T>,
//         start: Coordinate<T>,
//     ) -> StreamNodeClipFactory<EP, LINE, PR, PV, SINK, T> {
//         let line_node_factory = StreamNodeFactory::new(line.clone());

//         let ring_buffer: Buffer<T> = Buffer::default();
//         let line_node_buffer_factory = StreamNodeFactory::new(line);
//         let ring_sink_node = line_node_buffer_factory.generate(ring_buffer);

//         StreamNodeClipFactory {
//             ring_sink_node,
//             interpolate_fn,
//             line_node_factory,
//             phantom_pr: PhantomData::<PR>,
//             pv,
//             start,
//         }
//     }
// }

// impl<EP, LINE, PR, PV, SINK, T> NodeFactory for StreamNodeClipFactory<EP, LINE, PR, PV, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     LB: Line, LC: Line, LU: Line,
//     PR: ProjectionRawBase<T>,
//     PV: PointVisible<T = T>,
//     SINK: Stream<EP = EP, T = T>,
//     StreamNode<EP, LINE, SINK, T>: Stream<EP = EP, T = T>,
//     StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type Sink = SINK;
//     type T = T;
//     type Node = ClipNode<EP, LINE, PV, SINK, T>;

//     fn generate(&self, sink: Self::Sink) -> ClipNode<EP, LINE, PV, SINK, T> {
//         ClipNode::new(
//             self.pv.clone(),
//             self.line_node_factory.clone(),
//             self.interpolate_fn.clone(),
//             self.ring_sink_node.clone(),
//             sink,
//             self.start,
//         )
//     }
// }
