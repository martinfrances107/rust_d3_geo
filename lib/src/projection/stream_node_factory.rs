// use core::marker::PhantomData;
// use std::fmt::Debug;

// use geo::CoordFloat;
// use num_traits::FloatConst;

// use crate::stream::Stream;

// use super::NodeFactory;
// use super::StreamNode;

// /// Used in the construct of a Projection stream pipeline.
// ///
// /// RAW is the proto-node. ( The node without the link to other node's up the chain)
// ///
// /// RAW is precomputed and held in the projection.
// ///
// /// Inside Projection::stream() NodeFactory::generate() will be called to
// /// construct the pipeline.
// #[derive(Copy, Clone, Debug)]
// pub struct StreamNodeFactory<EP, RAW, SINK, T>
// where
//     RAW: Clone,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat,
// {
//     phantom_t: PhantomData<T>,
//     phantom_sink: PhantomData<SINK>,
//     raw: RAW,
// }

// impl<EP, RAW, SINK, T> StreamNodeFactory<EP, RAW, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     RAW: Clone,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat,
// {
//     pub(crate) fn new(raw: RAW) -> StreamNodeFactory<EP, RAW, SINK, T> {
//         StreamNodeFactory {
//             phantom_t: PhantomData::<T>,
//             phantom_sink: PhantomData::<SINK>,
//             raw,
//         }
//     }
// }

// // /// RAW here can be projection_raw or line_raw,
// /// as used by link_sink_factory.
// impl<EP, RAW, SINK, T> NodeFactory for StreamNodeFactory<EP, RAW, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     RAW: Clone,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     type Sink = SINK;
//     type T = T;
//     type Node = StreamNode<EP, RAW, Self::Sink, Self::T>;
//     fn generate(&self, sink: Self::Sink) -> Self::Node {
//         StreamNode {
//             raw: self.raw.clone(),
//             sink,
//         }
//     }
// }
