pub mod resample;
pub mod resample_none;

use std::fmt::Display;
use std::ops::AddAssign;
// use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::resample::resample::Resample;
use super::resample::resample_none::ResampleNone;
// use crate::clip::clip_sink_enum::ClipSinkEnum;
use crate::stream::stream_in_trait::StreamIn;
// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

pub trait ResampleTrait: Stream + StreamIn {}

// #[derive(Debug)]
// pub enum ResampleEnum<'a, PR, STREAM, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
//     STREAM: Stream<SC = Coordinate<T>>,
// {
//     RN(ResampleNone<'a, PR, STREAM, T>),
//     R(Resample<'a, PR, STREAM, T>),
// }

/// todo! find a better way.
// impl<'a, PR, STREAM, T> Stream for ResampleEnum<'a, PR, STREAM, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     STREAM: Stream<SC = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     type SC = Coordinate<T>;

//     // fn get_dst(
//     //     &self,
//     // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
//     // {
//     //     match self {
//     //         ResampleEnum::R(resample) => resample.get_dst(),
//     //         ResampleEnum::RN(rn) => rn.get_dst(),
//     //     }
//     // }
//     fn sphere(&mut self) {
//         match self {
//             ResampleEnum::R(resample) => resample.sphere(),
//             ResampleEnum::RN(rn) => rn.sphere(),
//         }
//     }
//     fn polygon_start(&mut self) {
//         match self {
//             ResampleEnum::R(resample) => resample.polygon_start(),
//             ResampleEnum::RN(rn) => rn.polygon_start(),
//         }
//     }
//     fn polygon_end(&mut self) {
//         match self {
//             ResampleEnum::R(resample) => resample.polygon_end(),
//             ResampleEnum::RN(rn) => rn.polygon_end(),
//         }
//     }
//     fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
//         match self {
//             ResampleEnum::R(resample) => resample.point(&*p, m),
//             ResampleEnum::RN(rn) => rn.point(p, m),
//         }
//     }
//     fn line_start(&mut self) {
//         match self {
//             ResampleEnum::R(resample) => resample.line_start(),
//             ResampleEnum::RN(rn) => rn.line_start(),
//         }
//     }
//     fn line_end(&mut self) {
//         match self {
//             ResampleEnum::R(resample) => resample.line_end(),
//             ResampleEnum::RN(rn) => rn.line_end(),
//         }
//     }
// }

// impl<'a, PR, STREAM, T> ResampleEnum<'a, PR, STREAM, T>
// where
//     PR: Transform<C = Coordinate<T>>,
//     Rc<PR>: Transform<C = Coordinate<T>>,
//     STREAM: Stream<SC = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     #[inline]
//     pub fn stream_in(&mut self, stream: STREAM) {
//         match self {
//             ResampleEnum::RN(s) => {
//                 s.stream_in(stream);
//             }
//             ResampleEnum::R(s) => {
//                 s.stream_in(stream);
//             }
//         }
//     }
// }

pub fn gen_resample_node<'a, PR, SInput: 'a, T>(
    projection_raw: &'a PR,
    delta2: T,
) -> Box<dyn ResampleTrait<SC = Coordinate<T>, SInput = SInput> + '_>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
    SInput: Stream<SC = Coordinate<T>> + Default,
{
    if delta2.is_zero() {
        Box::new(ResampleNone::new(projection_raw))
    } else {
        Box::new(Resample::new(projection_raw))
    }
}
