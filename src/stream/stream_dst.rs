// use std::fmt::Debug;
// use std::fmt::Display;
// use std::ops::AddAssign;
// use web_sys::CanvasRenderingContext2d;

// use geo::{CoordFloat, Coordinate};
// use num_traits::AsPrimitive;
// use num_traits::Float;
// use num_traits::FloatConst;

// use super::Stream;
// use super::StreamSourceDummy;
// use crate::centroid::centroid_stream::CentroidStream;
// use crate::circle::circle::CircleStream;
// use crate::length::LengthStream;
// use crate::path::bounds_stream::BoundsStream;
// use crate::path::path_area_stream::PathAreaStream;
// use crate::path::path_context_stream::PathContextStream;
// use crate::path::path_string::PathString;
// use crate::path::PathResult;
// use crate::path::PathResultEnum;

// pub trait StreamDst: Stream {
//     type T;
//     // type Out;
//     fn result<SD: StreamDst>(&mut self) -> SD;
// }
// #[derive(Debug)]
// pub enum StreamDst<'a, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + Float + FloatConst,
// {
//     Context2D(CanvasRenderingContext2d),
//     Circle(&'a CircleStream<T>),
//     BS(BoundsStream<T>),
//     CS(CentroidStream<T>),
//     LS(LengthStream<T>),
//     PAS(PathAreaStream<T>),
//     PathString(PathString<T>),
//     PathContextStream(PathContextStream<T>),
//     SRC(StreamSourceDummy<T>),
// }

// impl<'a, T> PathResult for StreamDst<'a, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     type Out = Option<PathResultEnum<T>>;
//     fn result(&mut self) -> Self::Out {
//         match self {
//             StreamDst::Context2D(_) => {
//                 // context.result()
//                 None
//             }
//             StreamDst::Circle(_) => None,
//             StreamDst::BS(bs) => bs.result(),
//             StreamDst::CS(_cs) => None,
//             StreamDst::LS(_ls) => None,
//             StreamDst::PAS(pas) => pas.result(),
//             StreamDst::PathContextStream(pcs) => pcs.result(),
//             StreamDst::PathString(ps) => ps.result(),
//             StreamDst::SRC(_) => {
//                 panic!("when calling result on a dumnmy value");
//             }
//         }
//     }
// }

// impl<'a, T> Stream for StreamDst<'a, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Debug + Default + Display + FloatConst,
// {
//     type SC = Coordinate<T>;

//     fn get_dst(&self) -> StreamDst<T> {
//         match self {
//             StreamDst::Context2D(_c) => {
//                 panic!("what todo here.");
//             }
//             StreamDst::PathContextStream(pas) => pas.get_dst(),
//             StreamDst::PAS(pas) => pas.get_dst(),
//             StreamDst::PathString(ps) => ps.get_dst(),
//             StreamDst::BS(bs) => bs.get_dst(),
//             StreamDst::CS(cs) => cs.get_dst(),
//             StreamDst::LS(ls) => ls.get_dst(),
//             StreamDst::Circle(c) => c.get_dst(),
//             StreamDst::SRC(_src) => {
//                 todo!("handle dummy")
//             }
//         }
//     }
//     fn sphere(&mut self) {
//         match self {
//             StreamDst::Context2D(_c) => {
//                 panic!("what todo here.");
//             }
//             StreamDst::PathContextStream(pas) => pas.sphere(),
//             StreamDst::PAS(pas) => pas.sphere(),
//             StreamDst::PathString(ps) => ps.sphere(),
//             StreamDst::BS(bs) => bs.sphere(),
//             StreamDst::CS(cs) => cs.sphere(),
//             StreamDst::LS(ls) => ls.sphere(),
//             StreamDst::Circle(c) => c.sphere(),
//             StreamDst::SRC(_src) => {
//                 todo!("handle dummy")
//             }
//         }
//     }
//     fn polygon_start(&mut self) {
//         match self {
//             StreamDst::Context2D(_c) => {
//                 panic!("what todo here.");
//             }
//             StreamDst::PathContextStream(pas) => pas.polygon_start(),
//             StreamDst::PAS(pas) => pas.polygon_start(),
//             StreamDst::PathString(ps) => ps.polygon_start(),
//             StreamDst::BS(bs) => bs.polygon_start(),
//             StreamDst::CS(cs) => cs.polygon_start(),
//             StreamDst::LS(ls) => ls.polygon_start(),
//             StreamDst::Circle(c) => c.polygon_start(),
//             StreamDst::SRC(_src) => {
//                 todo!("handle dummy")
//             }
//         }
//     }
//     fn polygon_end(&mut self) {
//         match self {
//             StreamDst::Context2D(_c) => {
//                 panic!("what todo here.");
//             }
//             StreamDst::PathContextStream(pas) => pas.polygon_end(),
//             StreamDst::PAS(pas) => pas.polygon_end(),
//             StreamDst::PathString(ps) => ps.polygon_end(),
//             StreamDst::BS(bs) => bs.polygon_end(),
//             StreamDst::CS(cs) => cs.polygon_end(),
//             StreamDst::LS(ls) => ls.polygon_end(),
//             StreamDst::Circle(c) => c.polygon_end(),
//             StreamDst::SRC(_src) => {
//                 todo!("handle dummy")
//             }
//         }
//     }
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             StreamDst::Context2D(_c) => {
//                 panic!("what todo here.");
//             }
//             StreamDst::PathContextStream(pas) => pas.point(p, m),
//             StreamDst::PAS(pas) => pas.point(p, m),
//             StreamDst::PathString(ps) => ps.point(p, m),
//             StreamDst::BS(bs) => bs.point(p, m),
//             StreamDst::CS(cs) => cs.point(p, m),
//             StreamDst::LS(ls) => ls.point(p, m),
//             StreamDst::Circle(c) => c.point(p, m),
//             StreamDst::SRC(_src) => {
//                 todo!("handle dummy")
//             }
//         }
//     }
//     fn line_start(&mut self) {
//         match self {
//             StreamDst::Context2D(_c) => {
//                 panic!("what todo here.");
//             }
//             StreamDst::PathContextStream(pas) => pas.line_start(),
//             StreamDst::PAS(pas) => pas.line_start(),
//             StreamDst::PathString(ps) => ps.line_start(),
//             StreamDst::BS(bs) => bs.line_start(),
//             StreamDst::CS(cs) => cs.line_start(),
//             StreamDst::LS(ls) => ls.line_start(),
//             StreamDst::Circle(c) => c.line_start(),
//             StreamDst::SRC(_src) => {
//                 todo!("handle dummy")
//             }
//         }
//     }
//     fn line_end(&mut self) {
//         match self {
//             StreamDst::Context2D(_c) => {
//                 panic!("what todo here.");
//             }
//             StreamDst::PathContextStream(pas) => pas.line_end(),
//             StreamDst::PAS(pas) => pas.line_end(),
//             StreamDst::PathString(ps) => ps.line_end(),
//             StreamDst::BS(bs) => bs.line_end(),
//             StreamDst::CS(cs) => cs.line_end(),
//             StreamDst::LS(ls) => ls.line_end(),
//             StreamDst::Circle(c) => c.line_end(),
//             StreamDst::SRC(_src) => {
//                 todo!("handle dummy")
//             }
//         }
//     }
// }
