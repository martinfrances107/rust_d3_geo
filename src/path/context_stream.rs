// use std::fmt::Display;
// use std::ops::AddAssign;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use super::context::Context;
// use super::string::String;
// use super::PointRadiusTrait;
// use super::Result;
// use super::ResultEnum;
// use crate::stream::Stream;

// #[derive(Clone, Debug)]
// pub enum ContextStream<T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     PC(Context<T>),
//     PS(String<T>),
// }

// impl<T> Result for ContextStream<T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type Out = Option<ResultEnum<T>>;
//     fn result(&mut self) -> Self::Out {
//         match self {
//             ContextStream::PC(pc) => pc.result(),
//             ContextStream::PS(ps) => ps.result(),
//         }
//     }
// }

// impl<T> PointRadiusTrait for ContextStream<T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type PrtT = Option<T>;
//     fn point_radius(&mut self, val: Self::PrtT) {
//         match self {
//             ContextStream::PC(pc) => pc.point_radius(val),
//             ContextStream::PS(ps) => ps.point_radius(val),
//         }
//     }
// }

// impl<T> Stream for ContextStream<T>
// where
//     T: CoordFloat + FloatConst + AddAssign + AsPrimitive<T> + Display,
// {
//     type SC = Coordinate<T>;

//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             ContextStream::PC(pc) => pc.point(p, m),
//             ContextStream::PS(ps) => ps.point(p, m),
//         }
//     }
//     fn sphere(&mut self) {
//         match self {
//             ContextStream::PC(pc) => pc.sphere(),
//             ContextStream::PS(ps) => ps.sphere(),
//         }
//     }
//     fn line_start(&mut self) {
//         match self {
//             ContextStream::PC(pc) => pc.line_start(),
//             ContextStream::PS(ps) => ps.line_start(),
//         }
//     }
//     fn line_end(&mut self) {
//         match self {
//             ContextStream::PC(pc) => pc.line_end(),
//             ContextStream::PS(ps) => ps.line_end(),
//         }
//     }
//     fn polygon_start(&mut self) {
//         match self {
//             ContextStream::PC(pc) => pc.polygon_start(),
//             ContextStream::PS(ps) => ps.polygon_start(),
//         }
//     }
//     fn polygon_end(&mut self) {
//         match self {
//             ContextStream::PC(pc) => pc.polygon_end(),
//             ContextStream::PS(ps) => ps.polygon_end(),
//         }
//     }
//     // fn get_dst(&self) -> Self {
//     //     match self {
//     //         ContextStream::PC(pc) => pc.get_dst(),
//     //         ContextStream::PS(ps) => ps.get_dst(),
//     //     }
//     // }
// }
