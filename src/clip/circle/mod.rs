mod intersect;
mod line;

use std::cell::RefCell;
use std::f64;
use std::rc::Rc;

use delaunator::Point;

use super::Clip;
use crate::circle::circle_stream::circle_stream;
use crate::clip::PointsVisibleFn;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use line::Line;

pub fn generate_circle(radius: f64) -> StreamProcessor {
    let cr = radius.cos();
    let delta = 6f64.to_radians();

    let visible: Rc<PointsVisibleFn> =
        Rc::new(Box::new(move |lambda: f64, phi: f64, _m: Option<f64>| {
            return lambda.cos() * phi.cos() > cr;
        }));

    let interpolate = move |from: Option<Point>,
                            to: Option<Point>,
                            direction: f64,
                            stream: Rc<RefCell<Box<dyn TransformStream>>>| {
        circle_stream(stream, radius, delta, direction, from, to)
    };

    let ccl: StreamProcessor = Line::new(visible.clone(), radius);
    let clip_line_fn_ptr: Rc<RefCell<StreamProcessor>>;
    clip_line_fn_ptr = Rc::new(RefCell::new(Box::new(ccl)));

    return Clip::new(
        visible,
        clip_line_fn_ptr,
        Rc::new(RefCell::new(Box::new(interpolate))),
        Point {
            x: -f64::consts::PI,
            y: -f64::consts::FRAC_PI_2,
        },
    );
}
