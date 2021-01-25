use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

mod interpolate;
mod intersect;
mod line;

use crate::transform_stream::StreamProcessor;

use super::Clip;
use super::ClipLineFn;

use interpolate::interpolate;
use line::Line;

pub fn point_visible<T>(_x: T, _y: T, _z: Option<u8>) -> bool {
    return true;
}

pub fn generate_antimeridian<T: CoordFloat + FloatConst + 'static>() -> StreamProcessor<T> {
    let cal = Line::new();

    let clip_line_fn_ptr: ClipLineFn<T>;
    clip_line_fn_ptr = cal;

    return Clip::new(
        Box::new(point_visible),
        clip_line_fn_ptr,
        Box::new(interpolate),
        Coordinate {
            x: -T::PI(),
            y: -T::FRAC_PI_2(),
        },
    );
}
