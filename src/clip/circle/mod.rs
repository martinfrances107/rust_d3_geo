mod intersect;
mod line;

use super::Clip;
use crate::circle::circle_stream::circle_stream;
use crate::clip::PointsVisibleFn;
use crate::stream::Stream;

// use crate::transform_stream::TransformStream;
use geo::{CoordFloat, Coordinate};
use line::Line;
use num_traits::FloatConst;

pub fn generate_circle<T: CoordFloat + FloatConst + 'static>(
    radius: T,
) -> Box<dyn Fn(Box<dyn Stream<T>>) -> Box<dyn Stream<T>>> {
    let cr = radius.cos();
    let delta = T::from(6u8).unwrap().to_radians();

    let visible: PointsVisibleFn<T> = Box::new(move |lambda: T, phi: T, _m: Option<u8>| {
        return lambda.cos() * phi.cos() > cr;
    });

    let interpolate = Box::new(
        move |from: Option<Coordinate<T>>,
              to: Option<Coordinate<T>>,
              direction: T,
              stream: Box<dyn Stream<T>>| {
            circle_stream(stream, radius, delta, direction, from, to)
        },
    );

    let ccl = Line::new(visible, radius);
    let clip_line_fn_ptr;
    clip_line_fn_ptr = Box::new(ccl);

    return Clip::new(
        visible,
        clip_line_fn_ptr,
        interpolate,
        Coordinate {
            x: -T::PI(),
            y: -T::FRAC_PI_2(),
        },
    );
}
