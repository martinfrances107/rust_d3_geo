use std::cell::RefCell;
use std::rc::Rc;

use delaunator::Point;

pub mod resample;
pub mod resample_none;

use resample::Resample;
use resample_none::ResampleNone;

use crate::transform_stream::StreamProcessor;
use crate::Transform;

pub fn gen_resample(
  project: Rc<RefCell<Box<dyn Transform>>>,
  delta2: Option<f64>,
) -> Rc<RefCell<StreamProcessor>>
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
