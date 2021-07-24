// use std::fmt::Display;
// use std::ops::AddAssign;
// use std::rc::Rc;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// // use crate::projection::projection_trait::ProjectionTrait;
// // use crate::stream::stream_dst::StreamDst;
// use crate::stream::Clean;
// use crate::stream::CleanEnum;
// use crate::stream::Stream;
// use crate::Transform;

// use super::antimeridian::line::Line as AntimeridianLine;
// use super::circle::line::Line as CircleLine;
// // use super::line_sink_enum::LineSinkEnum;
// // use crate::projection::ProjectionRawTrait;

// // #[derive(Debug)]
// pub enum LineEnum<'a, PR, T>
// where
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     PR: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     Antimeridian(AntimeridianLine<'a, PR, T>),
//     Circle(CircleLine<'a, PR, T>),
//     Blank,
// }

// impl<'a, PR, T> Default for LineEnum<'a, PR, T>
// where
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     PR: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     fn default() -> Self {
//         LineEnum::Blank
//     }
// }
// impl<'a, PR, T> LineEnum<'a, PR, T>
// where
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     PR: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     pub fn stream_in(&mut self, stream: LineSinkEnum<'a, PR, T>) {
//         match self {
//             LineEnum::Antimeridian(line) => line.stream_in(stream),
//             LineEnum::Circle(line) => line.stream_in(stream),
//             LineEnum::Blank => panic!("LineEnum stream_in Shoud not be injecting a blank"),
//         }
//     }
//     // deviation from javascript access to ring_buffer is through
//     // ring_sink!
//     #[inline]
//     pub fn get_stream(&'a mut self) -> &mut LineSinkEnum<'a, PR, T> {
//         match self {
//             LineEnum::Antimeridian(line) => line.get_stream(),
//             LineEnum::Circle(line) => line.get_stream(),
//             LineEnum::Blank => panic!("LineEnum get_stream Should not be returning from  a blank."),
//         }
//     }
// }

// impl<'a, PR, T> Clean for LineEnum<'a, PR, T>
// where
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     PR: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     fn clean(&self) -> CleanEnum {
//         match self {
//             LineEnum::Antimeridian(l) => l.clean(),
//             LineEnum::Circle(l) => l.clean(),
//             LineEnum::Blank => panic!("should not be cleaning a blank."),
//         }
//     }
// }

// impl<'a, PR, T> Stream for LineEnum<'a, PR, T>
// where
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     PR: Transform<C = Coordinate<T>>,
//     // SD: StreamDst,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     type SC = Coordinate<T>;
//     // type SD = SD;
//     // type ST = T;

//     #[inline]
//     // fn get_dst(
//     //     &self,
//     // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
//     // {
//     //     match self {
//     //         LineEnum::Antimeridian(antimeridian) => antimeridian.get_dst(),
//     //         LineEnum::Circle(circle) => circle.get_dst(),
//     //         LineEnum::Blank => panic!("blank has no destinaation"),
//     //     }
//     // }
//     #[inline]
//     fn sphere(&mut self) {
//         match self {
//             LineEnum::Antimeridian(antimeridian) => antimeridian.sphere(),
//             LineEnum::Circle(circle) => circle.sphere(),
//             LineEnum::Blank => panic!("blank -- sphere!"),
//         }
//     }

//     #[inline]
//     fn polygon_start(&mut self) {
//         match self {
//             LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_start(),
//             LineEnum::Circle(circle) => circle.polygon_start(),
//             LineEnum::Blank => panic!("blank -- polygon start!"),
//         }
//     }

//     #[inline]
//     fn polygon_end(&mut self) {
//         match self {
//             LineEnum::Antimeridian(antimeridian) => antimeridian.polygon_end(),
//             LineEnum::Circle(circle) => circle.polygon_end(),
//             LineEnum::Blank => panic!("blank -- polygon end!"),
//         }
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         match self {
//             LineEnum::Antimeridian(antimeridian) => antimeridian.line_start(),
//             LineEnum::Circle(circle) => circle.line_start(),
//             LineEnum::Blank => panic!("blank -- line start!"),
//         }
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         match self {
//             LineEnum::Antimeridian(antimeridian) => antimeridian.line_end(),
//             LineEnum::Circle(circle) => circle.line_end(),
//             LineEnum::Blank => panic!("blank -- line end!"),
//         }
//     }

//     #[inline]
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             LineEnum::Antimeridian(antimeridian) => antimeridian.point(p, m),
//             LineEnum::Circle(circle) => circle.point(p, m),
//             LineEnum::Blank => panic!("blank -- point!"),
//         }
//     }
// }
