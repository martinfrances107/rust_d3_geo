use std::fmt::Debug;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::circle::stream_fn::stream_fn;
use crate::clip::InterpolateFn;
use crate::stream::Stream;

/// Sets up a clip circle interpolate function, for a given radius.
pub fn generate<EP, STREAM, T>(radius: T) -> InterpolateFn<STREAM, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    STREAM: Stream<EP = EP, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    // let cr = radius.cos();
    let delta = T::from(6_f64).unwrap().to_radians();
    // let smallRadius = cr > T::zero();
    // notHemisphere = abs(cr) > epsilon; // TODO optimise for this common case

    let out: InterpolateFn<STREAM, T> = Rc::new(
        move |from: Option<Coordinate<T>>,
              to: Option<Coordinate<T>>,
              direction: T,
              stream: &mut STREAM| {
            stream_fn(stream, radius, delta, direction, from, to)
        },
    );

    out
}
