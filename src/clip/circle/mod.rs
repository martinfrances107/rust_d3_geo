mod intersect;
mod line;

use super::Clip;
use crate::clip::InterpolateFn;
use crate::clip::PointVisibleFn;
use crate::stream::StreamNode;
use crate::{circle::circle_stream::circle_stream, transform_stream::StreamProcessor};
use geo::{CoordFloat, Coordinate};
use line::Line;
use num_traits::FloatConst;
use std::rc::Rc;

/// Returns a clip object
pub fn generate_circle<T: CoordFloat + FloatConst + 'static>(radius: T) -> StreamProcessor<T> {
    let cr = radius.cos();
    let delta = T::from(6u8).unwrap().to_radians();

    let interpolate: InterpolateFn<T> = Rc::new(Box::new(
        move |from: Option<Coordinate<T>>,
              to: Option<Coordinate<T>>,
              direction: T,
              stream: StreamNode<T>| {
            circle_stream(stream, radius, delta, direction, from, to)
        },
    ));

    let visible: PointVisibleFn<T> = Rc::new(Box::new(move |lambda: T, phi: T, _m: Option<u8>| {
        return lambda.cos() * phi.cos() > cr;
    }));

    let ccl = Line::new(visible.clone(), radius);
    let clip_line_fn_ptr;
    clip_line_fn_ptr = Box::new(ccl);

    return Clip::gen_stream_processor(
        visible,
        clip_line_fn_ptr,
        interpolate,
        Coordinate {
            x: -T::PI(),
            y: -T::FRAC_PI_2(),
        },
    );
}
