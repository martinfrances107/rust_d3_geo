// use geo::{CoordFloat, Coordinate};
// use num_traits::FloatConst;

// use super::Stream;

// use super::Streamable;
// use geo::Line;

// impl<T: CoordFloat + FloatConst> Streamable<T> for Line<T> {
//     fn to_stream(&self, stream: &mut impl Stream<T>) {
//         // TODO there must be a better conversion.
//         stream.point(
//             Coordinate {
//                 x: self.start_point().x(),
//                 y: self.start_point().x(),
//             },
//             None,
//         );
//         stream.point(
//             Coordinate {
//                 x: self.end_point().x(),
//                 y: self.end_point().x(),
//             },
//             None,
//         );
//     }
// }
