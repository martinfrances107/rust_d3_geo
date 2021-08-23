use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::InterpolateFn;
use crate::stream::Stream;

// fn gen_interpolate<F>(radius: T) ->F
// where F: FnMut( Option<Coordinate<T>,  Option<Coordinate<T>>, T,  Rc<RefCell<STREAM>>)
// {
//     var cr = cos(radius),
//     delta = 6 * radians,
//     smallRadius = cr > 0,
//     notHemisphere = abs(cr) > epsilon; // TODO optimise for this common case

//     |from: Option<Coordinate<T>, to: Option<Coordinate<T>>, direction:T, stream: Rc<RefCell<STREAM>>| ->{
//         circleStream(stream, radius, delta, direction, from , to);
//     }
// }

/// Antimerdian interpolate function.
pub fn generate<STREAM, T>() -> InterpolateFn<STREAM, T>
where
    STREAM: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    let out: InterpolateFn<STREAM, T> = Rc::new(
        move |to: Option<Coordinate<T>>,
              from: Option<Coordinate<T>>,
              direction: T,
              stream_in: Rc<RefCell<STREAM>>| {
            let phi: T;
            let mut s = stream_in.borrow_mut();
            match from {
                None => {
                    phi = direction * T::FRAC_PI_2();

                    s.point(
                        &Coordinate {
                            x: -T::PI(),
                            y: phi,
                        },
                        None,
                    );
                    s.point(
                        &Coordinate {
                            x: T::zero(),
                            y: phi,
                        },
                        None,
                    );
                    s.point(&Coordinate { x: T::PI(), y: phi }, None);

                    s.point(
                        &Coordinate {
                            x: T::PI(),
                            y: T::zero(),
                        },
                        None,
                    );
                    s.point(
                        &Coordinate {
                            x: T::PI(),
                            y: -phi,
                        },
                        None,
                    );
                    s.point(
                        &Coordinate {
                            x: T::zero(),
                            y: -phi,
                        },
                        None,
                    );
                    s.point(
                        &Coordinate {
                            x: -T::PI(),
                            y: -phi,
                        },
                        None,
                    );
                    s.point(
                        &Coordinate {
                            x: -T::PI(),
                            y: T::zero(),
                        },
                        None,
                    );
                    s.point(
                        &Coordinate {
                            x: -T::PI(),
                            y: phi,
                        },
                        None,
                    );
                }
                Some(from) => {
                    let to = to.unwrap();
                    if (from.x - to.x).abs() > T::epsilon() {
                        let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                        phi = direction * lambda / T::from(2).unwrap();
                        s.point(&Coordinate { x: -lambda, y: phi }, None);
                        s.point(
                            &Coordinate {
                                x: T::zero(),
                                y: phi,
                            },
                            None,
                        );
                        s.point(&Coordinate { x: lambda, y: phi }, None);
                    } else {
                        s.point(&to, None);
                    }
                }
            }
        },
    );

    out
}
