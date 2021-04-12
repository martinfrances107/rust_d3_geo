pub mod resample;
pub mod resample_none;

use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::buffer::LineElem;
use crate::clip::clip::Clip;
use crate::clip::clip_raw::ClipRaw;
use crate::clip::clip_sink_enum::ClipSinkEnum;
use crate::compose::Compose;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;

use super::resample::resample::Resample;
use super::resample::resample_none::ResampleNone;

#[derive(Clone, Debug)]
pub enum ResampleEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    RN(ResampleNone<T>),
    R(Resample<T>),
}

/// todo! find a better way.
impl<T> Stream<T> for ResampleEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    type C = Coordinate<T>;

    fn get_dst(&self) -> StreamDst<T> {
        match self {
            ResampleEnum::R(resample) => resample.get_dst(),
            ResampleEnum::RN(rn) => rn.get_dst(),
        }
    }
    fn sphere(&mut self) {
        match self {
            ResampleEnum::R(resample) => resample.sphere(),
            ResampleEnum::RN(rn) => rn.sphere(),
        }
    }
    fn polygon_start(&mut self) {
        match self {
            ResampleEnum::R(resample) => resample.polygon_start(),
            ResampleEnum::RN(rn) => rn.polygon_start(),
        }
    }
    fn polygon_end(&mut self) {
        match self {
            ResampleEnum::R(resample) => resample.polygon_end(),
            ResampleEnum::RN(rn) => rn.polygon_end(),
        }
    }
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self {
            ResampleEnum::R(resample) => resample.point(&*p, m),
            ResampleEnum::RN(rn) => rn.point(p, m),
        }
    }
    fn line_start(&mut self) {
        match self {
            ResampleEnum::R(resample) => resample.line_start(),
            ResampleEnum::RN(rn) => rn.line_start(),
        }
    }
    fn line_end(&mut self) {
        match self {
            ResampleEnum::R(resample) => resample.line_end(),
            ResampleEnum::RN(rn) => rn.line_end(),
        }
    }
}
impl<T> ResampleEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    #[inline]
    pub fn stream_in(&mut self, stream: ClipSinkEnum<T>) {
        match self {
            ResampleEnum::RN(s) => {
                s.stream_in(stream);
            }
            ResampleEnum::R(s) => {
                s.stream_in(stream);
            }
        }
    }
}

pub fn gen_resample_node<T>(project: Compose<T>, delta2: T) -> ResampleEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    if delta2.is_zero() {
        ResampleEnum::RN(ResampleNone::new(project))
    } else {
        ResampleEnum::R(Resample {
            project: project,
            delta2,

            lambda00: T::zero(),
            x00: T::zero(),
            y00: T::zero(),
            a00: T::zero(),
            b00: T::zero(),
            c00: T::zero(), // first point

            lambda0: T::zero(),
            x0: T::zero(),
            y0: T::zero(),
            a0: T::zero(),
            b0: T::zero(),
            c0: T::zero(), // previous point
            cos_min_distance: (T::from(30f64).unwrap().to_radians()).cos(), // cos(minimum angular distance)

            stream: Box::new(ClipSinkEnum::Blank),
            use_line_point: true,
            use_line_end: true,
            use_line_start: true,
        })
    }
}
