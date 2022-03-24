/// Resample None.
pub mod none;
/// Resamples
pub mod resample;

use std::fmt::Debug;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use super::ProjectionRawBase;
// use crate::stream::{Connectable, Stream};
// use crate::stream::Connected;
// use crate::stream::Stream;
// use crate::stream::Unconnected;

// use none::None;
// use resample::Connected as ResampleConnected;
// use resample::Resample;
// use resample::Unconnected as ResampleUnconnected;

/// Applied to both resampler strategeries :-
/// None and Resample.
pub trait Resampler: Clone + Debug {}

// A return type which contains the
// generated resample node.
// #[derive(Debug, Clone)]
// pub enum ResampleNode<CS, PR, T>
// where
//     // EP:  Stream<EP = EP, T = T> + Default,
//     PR: ProjectionRawBase<T>,
//     // SINK: Stream<EP = EP, T = T>,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     /// Resample None.
//     RN(None<CS, PR, T>),
//     /// A Resample Node.
//     R(Resample<CS, PR, T>),
// }

// impl<PR, T> ResampleNode<Unconnected, PR, T>
// where
//     PR: ProjectionRawBase<T>,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     pub fn connect<EP, SINK>(self, sink: SINK) -> ResampleNode<Connected<SINK>, PR, T>
//     where
//         SINK: Stream<EP = EP, T = T>,
//     {
//         match self {
//             ResampleNode::RN(none) => ResampleNode::RN(none.connect(sink)),
//             ResampleNode::R(resample) => ResampleNode::R(resample.connect(sink)),
//         }
//     }
// }

// impl<PR, T> ResampleNode<ResampleUnconnected, PR, T>
// where
//     PR: ProjectionRawBase<T>,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     pub fn connect<EP, SINK>(self, sink: SINK) -> ResampleNode<ResampleConnected<SINK, T>, PR, T>
//     where
//         SINK: Stream<EP = EP, T = T>,
//     {
//         match self {
//             ResampleNode::RN(none) => ResampleNode::RN(none.connect(sink)),
//             ResampleNode::R(resample) => ResampleNode::R(resample.connect(sink)),
//         }
//     }
// }

// struct Resampler<CS>(CS);

// impl<'a, EP, PR, SINK, T> Stream for ResampleNode<Connected<SINK>, PR, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     PR: ProjectionRawBase<T>,
//     SINK: Stream<EP = EP, T = T>,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type EP = EP;
//     type T = T;

//     #[inline]
//     fn get_endpoint(self) -> Self::EP {
//         match self {
//             ResampleNode::RN(n) => n.get_endpoint(),
//             ResampleNode::R(r) => r.get_endpoint(),
//         }
//     }

//     #[inline]
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             ResampleNode::RN(n) => n.point(p, m),
//             ResampleNode::R(r) => r.point(p, m),
//         };
//     }

//     #[inline]
//     fn polygon_start(&mut self) {
//         match self {
//             ResampleNode::RN(n) => n.polygon_start(),
//             ResampleNode::R(r) => r.polygon_start(),
//         };
//     }

//     #[inline]
//     fn polygon_end(&mut self) {
//         match self {
//             ResampleNode::RN(n) => n.polygon_end(),
//             ResampleNode::R(r) => r.polygon_end(),
//         };
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         match self {
//             ResampleNode::RN(n) => n.line_start(),
//             ResampleNode::R(r) => r.line_start(),
//         };
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         match self {
//             ResampleNode::RN(n) => n.line_end(),
//             ResampleNode::R(r) => r.line_end(),
//         };
//     }

//     #[inline]
//     fn sphere(&mut self) {
//         match self {
//             ResampleNode::RN(n) => n.sphere(),
//             ResampleNode::R(r) => r.sphere(),
//         };
//     }
// }
