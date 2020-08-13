use std::cell::RefCell;
use std::rc::Rc;

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

pub mod resample;
pub mod resample_none;

use resample::Resample;
use resample_none::ResampleNone;

use crate::transform_stream::TransformStream;
use crate::Transform;

pub fn gen_resample<F>(
  project: Rc<RefCell<Box<dyn Transform<F>>>>,
  delta2: Option<F>,
) -> Rc<RefCell<Box<dyn TransformStream<F>>>>
where
  F: Float + FloatConst + FromPrimitive + 'static,
{
  match delta2 {
    Some(delta2) => {
      return Rc::new(RefCell::new(Box::new(Resample::new(project, delta2))));
    }
    None => {
      return Rc::new(RefCell::new(Box::new(ResampleNone::new(project))));
    }
  }
}
