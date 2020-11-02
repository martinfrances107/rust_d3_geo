use std::cell::RefCell;
use std::f64;
use std::rc::Rc;

use delaunator::Point;

mod interpolate;
mod intersect;
mod line;

use crate::transform_stream::StreamProcessor;

use super::Clip;

use interpolate::interpolate;
use line::Line;

pub fn point_visible(_x: f64, _y: f64, _z: Option<f64>) -> bool {
    return true;
}

pub fn generate_antimeridian() -> StreamProcessor {
    let cal: StreamProcessor = Line::new();

    let clip_line_fn_ptr: Rc<RefCell<StreamProcessor>>;
    clip_line_fn_ptr = Rc::new(RefCell::new(cal));

    return Clip::new(
        Rc::new(Box::new(point_visible)),
        clip_line_fn_ptr,
        Rc::new(RefCell::new(Box::new(interpolate))),
        Point {
            x: -f64::consts::PI,
            y: -f64::consts::FRAC_PI_2,
        },
    );
}
