use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

mod intersect;
mod line;
use crate::clip::PointVisibleFn;
use crate::transform_stream::StreamProcessor;

use super::{Clip, InterpolateFn};

use line::Line;
use std::rc::Rc;

use crate::stream::StreamNode;

#[inline]
pub fn generate_antimeridian<T: CoordFloat + FloatConst + 'static>() -> StreamProcessor<T> {
    let interpolate: InterpolateFn<T> = Rc::new(Box::new(
        move |from: Option<Coordinate<T>>,
              to: Option<Coordinate<T>>,
              direction: T,
              stream: StreamNode<T>| {
            let phi: T;
            let mut s = stream.borrow_mut();
            match from {
                None => {
                    phi = direction * T::FRAC_PI_2();
                    s.point(-T::PI(), phi, None);
                    s.point(T::zero(), phi, None);
                    s.point(T::PI(), phi, None);
                    s.point(T::PI(), T::zero(), None);
                    s.point(T::PI(), -phi, None);
                    s.point(T::zero(), -phi, None);
                    s.point(-T::PI(), -phi, None);
                    s.point(-T::PI(), T::zero(), None);
                    s.point(-T::PI(), phi, None);
                }
                Some(from) => {
                    // TODO investigate is to and Option<f64>
                    // let mut s = stream.borrow_mut();
                    let to = to.unwrap();
                    if (from.x - to.x).abs() > T::epsilon() {
                        let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                        phi = direction * lambda / T::from(2).unwrap();
                        s.point(-lambda, phi, None);
                        s.point(T::zero(), phi, None);
                        s.point(lambda, phi, None);
                    } else {
                        s.point(to.x, to.y, None);
                    }
                }
            }
        },
    ));
    let point_visible: PointVisibleFn<T> = Rc::new(Box::new(|_x: T, _y: T, _z: Option<u8>| true));
    Clip::gen_stream_processor(
        point_visible,
        Line::new(),
        interpolate,
        Coordinate {
            x: -T::PI(),
            y: -T::FRAC_PI_2(),
        },
    )
}
