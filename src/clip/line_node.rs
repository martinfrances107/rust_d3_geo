// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::FloatConst;

// use crate::clip::LineAntimeridian;
// use crate::clip::LineCircle;
// use crate::projection::stream_node::StreamNode;
// use crate::stream::Stream;

// #[derive(Clone, Debug)]
// pub enum LineNode<EP, SINK, T>
// where
//     EP: Clone + Debug + Stream<EP = EP, T = T>,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     C(StreamNode<EP, LineCircle<T>, SINK, T>),
//     A(StreamNode<EP, LineAntimeridian<T>, SINK, T>),
// }

// impl<EP, SINK, T> Stream for LineNode<EP, SINK, T>
// where
//     EP: Clone + Debug + Stream<EP = EP, T = T>,
//     SINK: Stream<EP = EP, T = T>,
//     T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type T = T;
//     type EP = EP;

//     #[inline]
//     fn get_endpoint(self) -> Self::EP {
//         match self {
//             LineNode::C(l) => l.get_endpoint(),
//             LineNode::A(l) => l.get_endpoint(),
//         }
//     }

//     #[inline]
//     fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
//         match self {
//             LineNode::C(l) => l.point(p, m),
//             LineNode::A(l) => l.point(p, m),
//         }
//     }

//     #[inline]
//     fn sphere(&mut self) {
//         match self {
//             LineNode::C(l) => l.sphere(),
//             LineNode::A(l) => l.sphere(),
//         }
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         match self {
//             LineNode::C(l) => l.line_start(),
//             LineNode::A(l) => l.line_start(),
//         }
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         match self {
//             LineNode::C(l) => l.line_end(),
//             LineNode::A(l) => l.line_end(),
//         }
//     }

//     #[inline]
//     fn polygon_start(&mut self) {
//         match self {
//             LineNode::C(l) => l.polygon_start(),
//             LineNode::A(l) => l.polygon_start(),
//         }
//     }

//     #[inline]
//     fn polygon_end(&mut self) {
//         match self {
//             LineNode::C(l) => l.polygon_end(),
//             LineNode::A(l) => l.polygon_end(),
//         }
//     }
// }
