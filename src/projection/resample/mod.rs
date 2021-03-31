pub mod resample;
pub mod resample_none;

// use crate::stream::stream_postclip_node_stub::StreamPostClipNodeStub;
// use crate::stream::CompareIntersection;
// use crate::stream::StreamPostClipTrait;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

// use resample_none::ResampleNone;
// use geo::CoordFloat;
// use num_traits::FloatConst;
// use resample::Resample;
// use resample_none::ResampleNone;

// use crate::stream::StreamProcessor;
use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::clip::ClipRaw;
use crate::compose::Compose;
use crate::stream::Stream;
use crate::stream::StreamDst;
use crate::Transform;

use super::resample::resample::Resample;
use super::resample::resample_none::ResampleNone;

// pub fn gen_resample<T>(project: Rc<Box<dyn Transform<>>>, delta2: Option<T>) -> StreamProcessor<T>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     return match delta2 {
//         Some(delta2) => Resample::new(project, delta2),
//         None => ResampleNone::new(project),
//     };
// }

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
    pub fn stream_in(&mut self, stream: Clip<T>) {
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

pub fn gen_resample_node<T>(
    // project: Box<dyn TransformClone<TcC = Coordinate<T>>>,
    project: Compose<T>,
    delta2: Option<T>,
) -> ResampleEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    match delta2 {
        None => ResampleEnum::RN(ResampleNone::new(project)),
        Some(delta2) => {
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

                stream: Box::new(Clip::new(
                    ClipRaw::Antimeridian(ClipAntimeridian::default()),
                    Coordinate::default(),
                )),
                use_line_point: true,
                use_line_end: true,
                use_line_start: true,
            })
        }
    }
}
