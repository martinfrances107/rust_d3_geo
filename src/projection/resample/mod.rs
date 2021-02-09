use std::rc::Rc;

pub mod resample;
pub mod resample_none;

// use geo::CoordFloat;
// use num_traits::FloatConst;
// use resample::Resample;
// use resample_none::ResampleNone;

// use crate::stream::StreamProcessor;
// use crate::Transform;

// pub fn gen_resample<T>(project: Rc<Box<dyn Transform<T>>>, delta2: Option<T>) -> StreamProcessor<T>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     return match delta2 {
//         Some(delta2) => Resample::new(project, delta2),
//         None => ResampleNone::new(project),
//     };
// }
