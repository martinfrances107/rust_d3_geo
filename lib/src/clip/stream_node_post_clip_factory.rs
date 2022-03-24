// use core::marker::PhantomData;
// use std::fmt::Debug;

// use derivative::*;
// use geo::CoordFloat;
// use num_traits::FloatConst;

// use super::post_clip_node::PostClipNode;

// use crate::projection::NodeFactory;
// use crate::stream::Stream;

// use super::post_clip::PostClip;

// /// Used in the construct of a Projection stream pipeline.
// ///
// /// Stream Raw (SR) is the proto-node. ( The node without the link to other node's up the chain)
// ///
// /// SR is precomputed and held in the projection.
// ///
// /// Inside Projection::stream() NodeFactory::generate() will be called to
// /// construct the pipeline.
// #[derive(Clone, Derivative)]
// #[derivative(Debug)]
// pub struct StreamNodePostClipFactory<SINK, T>
// where
//     T: CoordFloat + FloatConst,
// {
//     phantom_sink: PhantomData<SINK>,
//     post_clip: PostClip<T>,
// }

// impl<SINK, T> StreamNodePostClipFactory<SINK, T>
// where
//     T: CoordFloat + FloatConst,
// {
//     /// Given a PostClip construct a StreamNode.
//     #[inline]
//     pub fn new(post_clip: PostClip<T>) -> StreamNodePostClipFactory<SINK, T> {
//         StreamNodePostClipFactory {
//             phantom_sink: PhantomData::<SINK>,
//             post_clip,
//         }
//     }
// }

// impl<EP, SINK, T> NodeFactory for StreamNodePostClipFactory<SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     type Sink = SINK;
//     /// f32 or f64.
//     type T = T;
//     type Node = PostClipNode<EP, SINK, Self::T>;

//     #[inline]
//     fn generate(&self, sink: SINK) -> Self::Node {
//         match &self.post_clip {
//             PostClip::I(i) => PostClipNode::I(StreamNode {
//                 raw: i.clone(),
//                 sink,
//             }),
//             PostClip::R(r) => PostClipNode::R(StreamNode {
//                 raw: r.clone(),
//                 sink,
//             }),
//         }
//     }
// }
