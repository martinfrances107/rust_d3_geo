mod intersect;
mod line;

use std::cell::RefCell;
use std::rc::Rc;

use super::Clip;
use crate::circle::circle_stream::circle_stream;
use crate::clip::PointsVisibleFn;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use geo::Coordinate;
use line::Line;
use num_traits::{float::Float, FloatConst};

pub fn generate_circle<T: Float + FloatConst + 'static>(radius: T) -> StreamProcessor<T> {
    let cr = radius.cos();
    let delta = T::from(6u8).unwrap().to_radians();

    let visible: Rc<PointsVisibleFn<T>> =
        Rc::new(Box::new(move |lambda: T, phi: T, _m: Option<T>| {
            return lambda.cos() * phi.cos() > cr;
        }));

    let interpolate =
        move |from: Option<Coordinate<T>>,
              to: Option<Coordinate<T>>,
              direction: T,
              stream: Rc<RefCell<Box<dyn TransformStream<T>>>>| {
            circle_stream(stream, radius, delta, direction, from, to)
        };

    let ccl: StreamProcessor<T> = Line::new(visible.clone(), radius);
    let clip_line_fn_ptr: Rc<RefCell<StreamProcessor<T>>>;
    clip_line_fn_ptr = Rc::new(RefCell::new(Box::new(ccl)));

    return Clip::new(
        visible,
        clip_line_fn_ptr,
        Rc::new(RefCell::new(Box::new(interpolate))),
        Coordinate {
            x: -T::PI(),
            y: -T::FRAC_PI_2(),
        },
    );
}
