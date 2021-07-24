// use std::fmt::Display;
// use std::ops::AddAssign;
// use std::rc::Rc;

// use geo::CoordFloat;
// use geo::Coordinate;

// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use super::clip_buffer::ClipBuffer;
// // use super::clip_sink_enum::ClipSinkEnum;
// use crate::path::PathResult;
// use crate::path::PathResultEnum;
// // use crate::projection::ProjectionRawTrait;
// // use crate::stream::stream_dst::StreamDst;
// use crate::stream::Stream;
// use crate::Transform;
// // #[derive(Debug)]
// pub enum LineSinkEnum<'a, PR, T>
// where
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     PR: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     CSE(Box<ClipSinkEnum<'a, PR, T>>),
//     CB(ClipBuffer<T>),
// }

// impl<'a, PR, T> LineSinkEnum<'a, PR, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     #[inline]
//     pub fn result(&mut self) -> Option<PathResultEnum<T>> {
//         match self {
//             LineSinkEnum::CB(l) => l.result(),
//             LineSinkEnum::CSE(_) => {
//                 panic!("Calling result on a none buffer");
//             }
//         }
//     }
// }

// impl<'a, PR, T> Stream for LineSinkEnum<'a, PR, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     type SC = Coordinate<T>;
//     // type SD = SD;
//     // type ST = T;

//     #[inline]
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             LineSinkEnum::CSE(cse) => cse.point(p, m),
//             LineSinkEnum::CB(cb) => cb.point(p, m),
//         }
//     }

//     #[inline]
//     fn sphere(&mut self) {
//         match self {
//             LineSinkEnum::CSE(cse) => cse.sphere(),
//             LineSinkEnum::CB(cb) => cb.sphere(),
//         }
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         match self {
//             LineSinkEnum::CSE(cse) => cse.line_start(),
//             LineSinkEnum::CB(cb) => cb.line_start(),
//         }
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         match self {
//             LineSinkEnum::CSE(cse) => cse.line_end(),
//             LineSinkEnum::CB(cb) => cb.line_end(),
//         }
//     }

//     #[inline]
//     fn polygon_start(&mut self) {
//         match self {
//             LineSinkEnum::CSE(cse) => cse.sphere(),
//             LineSinkEnum::CB(cb) => cb.sphere(),
//         }
//     }

//     #[inline]
//     fn polygon_end(&mut self) {
//         match self {
//             LineSinkEnum::CSE(cse) => cse.sphere(),
//             LineSinkEnum::CB(cb) => cb.sphere(),
//         }
//     }

//     // #[inline]
//     // fn get_dst(
//     //     &self,
//     // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
//     // {
//     //     match self {
//     //         LineSinkEnum::CSE(cse) => cse.get_dst(),
//     //         LineSinkEnum::CB(cb) => cb.get_dst(),
//     //     }
//     // }
// }
