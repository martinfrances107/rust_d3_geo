// use std::fmt::Debug;

// use geo::CoordFloat;
// use num_traits::FloatConst;

// use crate::stream::Stream;

// /// Stream node is a internal to projection and clip.
// ///
// /// The stream node processor is the
// ///
// /// raw: the proto-node.
// /// sink: the next streamNode in the chain.
// ///
// /// T is required because SINK: Stream<T=T>
// #[derive(Clone, Debug)]
// pub struct StreamNode<EP, RAW, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     /// The proto node, that is the struct without reference to the sink.
//     pub raw: RAW,
//     /// The downstream node.
//     pub sink: SINK,
// }

// impl<EP, RAW, SINK, T> StreamNode<EP, RAW, SINK, T>
// where
//     EP:  Stream<EP = EP, T = T> + Default,
//     SINK: Stream<EP = EP, T = T>,
//     T: CoordFloat + FloatConst,
// {
//     /// Returns the end of the stream pipeline, the dain.
//     #[inline]
//     pub fn get_endpoint(self) -> EP {
//         self.sink.get_endpoint()
//     }
// }
