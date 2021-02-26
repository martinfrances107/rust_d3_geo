pub mod resample;
pub mod resample_none;

use crate::stream::StreamDummy;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use resample::Resample;
use resample_none::ResampleNone;
// use geo::CoordFloat;
// use num_traits::FloatConst;
// use resample::Resample;
// use resample_none::ResampleNone;

// use crate::stream::StreamProcessor;
use crate::stream::Stream;
use crate::Transform;

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
pub enum ResampleNode<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    Simple(ResampleNone<T>),
    Complex(Resample<T>),
}

pub trait StreamResampleTrait {
    type SRTsci; // Stream. Resample. Trait. stream clip in
    fn stream_postclip_in(&mut self, stream_clip_in: Self::SRTsci);
    // fn clone_box(&self) -> ResampleNode;
}

impl<T> StreamResampleTrait for ResampleNode<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type SRTsci = Box<dyn Stream<ScC = Coordinate<T>>>;
    fn stream_postclip_in(&mut self, stream_clip_in: Self::SRTsci) {
        // match self {
        //     ResampleNode::Simple(s) => {
        //         s.stream_post_clip_in(stream_clip_in);
        //     }
        //     ResampleNode::Complex(s) => {
        //         s.stream_post_clip_in(stream_clip_in);
        //     }
        // }
    }
}

pub fn gen_resample_node<T>(
    project: Box<dyn Transform<TcC = Coordinate<T>>>,
    delta2: Option<T>,
) -> ResampleNode<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    match delta2 {
        None => ResampleNode::Simple(ResampleNone::new(project.clone_box())),
        Some(delta2) => {
            ResampleNode::Complex(Resample {
                project: project.clone_box(),
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

                stream: Box::new(StreamDummy::default()),
                use_line_point: true,
                use_line_end: true,
                use_line_start: true,
            })
        }
    }
}
