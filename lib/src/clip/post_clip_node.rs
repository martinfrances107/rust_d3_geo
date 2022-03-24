// use std::fmt::Debug;

// use approx::AbsDiffEq;
// use geo::{CoordFloat, Coordinate};
// use num_traits::FloatConst;

// use crate::clip::rectangle::Rectangle;
// use crate::identity::Identity;
// use crate::stream::Connectable;
// use crate::stream::Connected;

// use crate::stream::Stream;
// use crate::stream::Unconnected;

// Marker trait applied to
// Identity (acts as pass through)
// Rectangle

// /// A Stream pipeline stage.
// #[derive(Clone, Debug)]
// pub enum PostClipNode<EP, SINK, STATE, T>
// where
//     EP: Stream<EP = EP, T = T> + Default,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat,
// {
//     /// Default pass thru.
//     I(Identity<EP, SINK, STATE, T>),
//     /// Clipping rectangle.
//     R(Rectangle<EP, SINK, STATE, T>),
// }

// impl<EP, SINK, T> Connectable for PostClipNode<EP, SINK, Unconnected, T>
// where
//     T: CoordFloat,
//     SINK: Stream<EP = EP, T = T>,
//     EP: Stream<EP = EP, T = T> + Default,
// {
//     type EP = EP;
//     type Output = PostClipNode<EP, EP, Unconnected, T>;
//     type Sink = PostClipNode<EP, EP, Unconnected, T>;
//     fn connect(self, sink: SINK) {}
// }

// impl<'a, EP, SINK, T> Stream for PostClipNode<EP, EP, Connected<SINK>, T>
// where
//     EP: Stream<EP = EP, T = T> + Default,
//     SINK: Stream<EP = EP, T = T>,
//     T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
//     type T = T;
//     type EP = EP;

//     #[inline]
//     fn get_endpoint(self) -> Self::EP {
//         match self {
//             PostClipNode::I(i) => i.get_endpoint(),
//             PostClipNode::R(r) => r.get_endpoint(),
//         }
//     }

//     fn sphere(&mut self) {
//         match self {
//             PostClipNode::I(i) => i.sphere(),
//             PostClipNode::R(r) => r.sphere(),
//         };
//     }
//     fn polygon_start(&mut self) {
//         match self {
//             PostClipNode::I(i) => i.polygon_start(),
//             PostClipNode::R(r) => r.polygon_start(),
//         };
//     }
//     fn polygon_end(&mut self) {
//         match self {
//             PostClipNode::I(i) => i.polygon_end(),
//             PostClipNode::R(r) => r.polygon_end(),
//         };
//     }
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             PostClipNode::I(i) => i.point(p, m),
//             PostClipNode::R(r) => r.point(p, m),
//         };
//     }
//     fn line_start(&mut self) {
//         match self {
//             PostClipNode::I(i) => i.line_start(),
//             PostClipNode::R(r) => r.line_start(),
//         };
//     }
//     fn line_end(&mut self) {
//         match self {
//             PostClipNode::I(i) => i.line_end(),
//             PostClipNode::R(r) => r.line_end(),
//         };
//     }
// }
