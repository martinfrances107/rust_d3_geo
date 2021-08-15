use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::circle::streamFn::streamFn;
use crate::clip::InterpolateFn;
use crate::stream::Stream;

pub fn generate<STREAM, T>(radius: T) -> InterpolateFn<STREAM, T>
where
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    // let cr = radius.cos();
    let delta = T::from(6_f64).unwrap().to_radians();
    // let smallRadius = cr > T::zero();
    // notHemisphere = abs(cr) > epsilon; // TODO optimise for this common case

    let out: InterpolateFn<STREAM, T> = Rc::new(
        move |from: Option<Coordinate<T>>,
              to: Option<Coordinate<T>>,
              direction: T,
              stream: Rc<RefCell<STREAM>>| {
            streamFn(stream, radius, delta, direction, from, to)
        },
    );

    out
}
