// use geo::{CoordFloat, Coordinate};
// use num_traits::FloatConst;

// use crate::stream::Stream;
// // use crate::transform_stream::StreamProcessor;
// use crate::Transform;

// // function resampleNone(project) {
// //   return transformer({
// //     point: f64unction(x, y) {
// //       x = project(x, y);
// //       this.stream.point(x[0], x[1]);
// //     }
// //   });
// // }

// pub struct ResampleNone<T> {
//     project: Box<dyn Transform<T>>,
//     stream: Box<dyn Stream<T>>,
// }

// impl<T: CoordFloat + FloatConst + 'static> ResampleNone<T> {
//     pub fn new(project: Box<dyn Transform<T>>) -> Box<dyn Fn(Box<dyn Stream<T>>) -> Box<Self>> {
//         return Box::new(move |stream: Box<dyn Stream<T>>| {
//             return Box::new(Self { project, stream });
//         });
//     }
// }

// impl<T: CoordFloat + FloatConst> Stream<T> for ResampleNone<T> {
//     fn point(&mut self, x: T, y: T, m: Option<u8>) {
//         let mut stream = self.stream;
//         let project = &*self.project;
//         let p = project.transform(&Coordinate { x, y });
//         stream.point(p.x, p.y, m);
//     }
// }
