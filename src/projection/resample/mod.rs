pub mod resample;
pub mod resample_none;

// use crate::stream::stream_dummy::StreamDummy;
use crate::stream::stream_postclip_node_stub::StreamPostClipNodeStub;
// use crate::stream::CompareIntersection;
// use crate::stream::StreamPostClipTrait;
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
use crate::stream::Stream;
// use crate::stream::StreamSrc;
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

#[derive(Clone)]
pub enum ResampleEnum<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    RN(ResampleNone<T>),
    R(Resample<T>),
}

/// todo! find a better way.
impl<T> Stream for ResampleEnum<T>
where
    T: CoordFloat + FloatConst + Default,
{
    type C = Coordinate<T>;
    fn point(&mut self, p: Self::C, m: Option<u8>) {
        match self {
            ResampleEnum::R(resample) => resample.point(p, m),
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

pub trait StreamResampleTrait {
    type SRTsci; // Stream. Resample. Trait. stream clip in
    fn stream_postclip_in(&mut self, stream_clip_in: Self::SRTsci);
    // fn box_clone(&self) -> ResampleNode;
}

impl<T> StreamResampleTrait for ResampleEnum<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    // type SRTsci = Box<
    //     dyn StreamPostClipTrait<
    //         SpostctStream = StreamSrc,
    //         C = Coordinate<T>,
    //         SctC = Coordinate<T>,
    //         SctT = T,
    //         SctOC = Option<Coordinate<T>>,
    //         SctCi = CompareIntersection<T>,
    //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
    //     >,
    // >;
    type SRTsci = Clip<T>;
    fn stream_postclip_in(&mut self, stream_clip_in: Self::SRTsci) {
        // match &mut self {
        //     ResampleNode::Simple(s) => {
        //         s.stream_post_clip_in(stream_clip_in);
        //     }
        //     ResampleNode::Complex(s) => {StreamResampleTrait
        //         s.stream_post_clip_in(stream_clip_in);
        //     }
        // }
    }
}

pub fn gen_resample_node<T>(
    project: Box<dyn Transform<TcC = Coordinate<T>>>,
    delta2: Option<T>,
) -> ResampleEnum<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    match delta2 {
        None => ResampleEnum::RN(ResampleNone::new(project.box_clone())),
        Some(delta2) => {
            ResampleEnum::R(Resample {
                project: project.box_clone(),
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
