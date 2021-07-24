// use std::fmt::Display;
// use std::ops::AddAssign;
// use std::rc::Rc;

// // use derivative::derivative;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// // use crate::projection::projection_trait::ProjectionTrait;
// use crate::projection::resample::gen_resample_node;
// use crate::projection::resample::ResampleEnum;
// // use crate::projection::ProjectionRawTrait;
// // use crate::stream::stream_dst::StreamDst;
// use crate::clip::interpolate_trait::Interpolate;
// use crate::stream::Stream;
// use crate::Transform;

// /// Wrapper for stream inputs to Clip.
// // #[derive(Debug)]
// pub enum ClipSinkEnum<'a, PR, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     Resample(ResampleEnum<'a, PR, T>),
//     Src(&'a mut Box<dyn Stream<SC = Coordinate<T>>>),
// }

// impl<'a, PR, T> ClipSinkEnum<'a, PR, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     pub fn new(projection_raw: Rc<PR>) -> Self {
//         ClipSinkEnum::Resample(gen_resample_node(projection_raw, T::zero()))
//     }
// }

// impl<'a, PR, T> Stream for ClipSinkEnum<'a, PR, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     type SC = Coordinate<T>;
//     // type ST = T;
//     // type SD = SD;
//     // #[inline]
//     // fn get_dst(&self) -> Self {
//     //     match self {
//     //         ClipSinkEnum::Resample(r) => r.get_dst(),
//     //         ClipSinkEnum::Src(s) => s.get_dst(),
//     //         ClipSinkEnum::Blank => panic!("calling get_dst on a blank"),
//     //     }
//     // }

//     #[inline]
//     fn sphere(&mut self) {
//         match self {
//             ClipSinkEnum::Resample(r) => r.sphere(),
//             ClipSinkEnum::Src(s) => s.sphere(),
//             // ClipSinkEnum::Blank => panic!("calling sphere on a blank"),
//         }
//     }

//     #[inline]
//     fn polygon_start(&mut self) {
//         match self {
//             ClipSinkEnum::Resample(r) => r.polygon_start(),
//             ClipSinkEnum::Src(s) => s.polygon_start(),
//             // ClipSinkEnum::Blank => panic!("calling polygon_start on a blank"),
//         }
//     }

//     #[inline]
//     fn polygon_end(&mut self) {
//         match self {
//             ClipSinkEnum::Resample(r) => r.polygon_end(),
//             ClipSinkEnum::Src(s) => s.polygon_end(),
//             // ClipSinkEnum::Blank => panic!("calling polygon_end on a blank"),
//         }
//     }

//     #[inline]
//     fn line_start(&mut self) {
//         match self {
//             ClipSinkEnum::Resample(r) => r.line_start(),
//             ClipSinkEnum::Src(s) => s.line_start(),
//             // ClipSinkEnum::Blank => panic!("calling line_start on a blank"),
//         }
//     }

//     #[inline]
//     fn line_end(&mut self) {
//         match self {
//             ClipSinkEnum::Resample(r) => r.line_end(),
//             ClipSinkEnum::Src(s) => s.line_end(),
//             // ClipSinkEnum::Blank => panic!("calling line_end on a blank"),
//         }
//     }

//     #[inline]
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             ClipSinkEnum::Resample(r) => r.point(p, m),
//             ClipSinkEnum::Src(s) => s.point(p, m), // LineEnum::Stub => panic!("calling point on a stub!"),
//                                                    // ClipSinkEnum::Blank => panic!("calling point on a blank"),
//         }
//     }
//     //// Todo must connect up other stream functions. or find a better way.
// }
