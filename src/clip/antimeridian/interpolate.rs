use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::InterpolateFn;
use crate::math::EPSILON;
use crate::stream::Stream;

/// Antimerdian interpolate function.
pub fn generate<EP, STREAM, T>() -> InterpolateFn<STREAM, T>
where
    STREAM: Stream<EP = EP, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    let epsilon = T::from(EPSILON).unwrap();
    let out: InterpolateFn<STREAM, T> = Rc::new(
        move |to: Option<Coordinate<T>>,
              from: Option<Coordinate<T>>,
              direction: T,
              stream_in: STREAM| {
            let phi: T;
            let mut s = stream_in;
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
                    if (from.x - to.x).abs() > epsilon {
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
