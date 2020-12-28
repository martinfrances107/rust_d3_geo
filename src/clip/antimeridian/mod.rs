use std::cell::RefCell;
use std::rc::Rc;

use geo::Coordinate;

use num_traits::{float::Float, FloatConst};

mod interpolate;
mod intersect;
mod line;

use crate::transform_stream::StreamProcessor;

use super::Clip;

use interpolate::interpolate;
use line::Line;

pub fn point_visible<T>(_x: T, _y: T, _z: Option<T>) -> bool {
    return true;
}

pub fn generate_antimeridian<T: Float + FloatConst + 'static>() -> StreamProcessor<T> {
    let cal: StreamProcessor<T> = Line::new();

    let clip_line_fn_ptr: Rc<RefCell<StreamProcessor<T>>>;
    clip_line_fn_ptr = Rc::new(RefCell::new(cal));

    return Clip::new(
        Rc::new(Box::new(point_visible)),
        clip_line_fn_ptr,
        Rc::new(RefCell::new(Box::new(interpolate))),
        Coordinate {
            x: -T::PI(),
            y: -T::FRAC_PI_2(),
        },
    );
}
