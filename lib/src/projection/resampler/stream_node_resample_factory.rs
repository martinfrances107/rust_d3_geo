// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use core::marker::PhantomData;
// use derivative::*;
// use geo::CoordFloat;
// use num_traits::FloatConst;

// use crate::compose::Compose;
// use crate::projection::resampler::Resample;
// use crate::projection::stream_node::StreamNode;
// use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
// use crate::projection::NodeFactory;
// use crate::projection::ProjectionRawBase;
// use crate::stream::Stream;

// use super::none::None as ResampleNone;
// use super::ResampleNode;

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
// pub struct StreamNodeResampleFactory<PR, SINK, T>
// where
//     PR: ProjectionRawBase<T>,
//     T: CoordFloat + FloatConst,
// {
//     phantom_sink: PhantomData<SINK>,
//     projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
//     delta2: T,
// }

// impl<PR, SINK, T> StreamNodeResampleFactory<PR, SINK, T>
// where
//     PR: ProjectionRawBase<T>,
//     T: CoordFloat + FloatConst,
// {
//     /// Given a transform and precision return the appropiate
//     /// resample factory.
//     ///
//     /// delta2 is the square of the sampling precision.
//     #[inline]
//     pub fn new(
//         projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
//         delta2: T,
//     ) -> StreamNodeResampleFactory<PR, SINK, T> {
//         StreamNodeResampleFactory {
//             delta2,
//             projection_transform,
//             phantom_sink: PhantomData::<SINK>,
//         }
//     }
// }

// impl<EP, PR, SINK, T> NodeFactory for StreamNodeResampleFactory<PR, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     PR: ProjectionRawBase<T>,
//     SINK: Stream<EP = EP, T = T>,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type Sink = SINK;
//     type T = T;
//     type Node = ResampleNode<EP, PR, Self::Sink, Self::T>;

//     #[inline]
//     fn generate(&self, sink: Self::Sink) -> Self::Node {
//         match self.delta2.is_zero() {
//             true => ResampleNode::RN(StreamNode {
//                 raw: ResampleNone::new(self.projection_transform.clone()),
//                 sink,
//             }),
//             false => ResampleNode::R(StreamNode {
//                 raw: Resample::new(self.projection_transform.clone(), self.delta2),
//                 sink,
//             }),
//         }
//     }
// }
