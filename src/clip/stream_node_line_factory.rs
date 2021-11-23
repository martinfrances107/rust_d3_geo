// use core::marker::PhantomData;
// use std::fmt::Debug;

// use geo::CoordFloat;
// use num_traits::FloatConst;

// use super::line::Line;
// use crate::projection::stream_node::StreamNode;
// use crate::projection::NodeFactory;
// use crate::stream::Stream;

// /// Used in the construct of a Projection stream pipeline.
// ///
// /// Stream Raw (SR) is the proto-node. ( The node without the link to other node's up the chain)
// ///
// /// SR is precomputed and held in the projection.
// ///
// /// Inside Projection::stream() NodeFactory::generate() will be called to
// /// construct the pipeline.
// #[derive(Clone, Debug)]
// pub struct StreamNodeLineFactory<EP, SINK, T>
// where
//     T: CoordFloat + FloatConst,
//     SINK: Stream<EP = EP, T = T>,
// {
//     phantom_sink: PhantomData<SINK>,
//     line: Line<T>,
// }

// impl<EP, SINK, T> StreamNodeLineFactory<EP, SINK, T>
// where
//     T: CoordFloat + FloatConst,
//     SINK: Stream<EP = EP, T = T>,
// {
//     /// Given a PostClip construct a StreamNode.
//     pub fn new(line: Line<T>) -> StreamNodeLineFactory<EP, SINK, T> {
//         StreamNodeLineFactory {
//             phantom_sink: PhantomData::<SINK>,
//             line,
//         }
//     }
// }

// impl<EP, SINK, T> NodeFactory for StreamNodeLineFactory<EP, SINK, T>
// where
//     EP: Clone + Debug + Stream<EP = EP, T = T>,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     type Sink = SINK;
//     // type Node = PostClipNode<SINK, T>;
//     type T = T;
//     type Node = StreamNode<EP, LINE, SINK, Self::T>;
//     fn generate(&self, sink: SINK) -> Self::Node {
//         match &self.line {
//             Line::A(l) => LineNode::A(StreamNode { raw: *l, sink }),
//             Line::C(l) => LineNode::C(StreamNode { raw: *l, sink }),
//         }
//     }
// }
