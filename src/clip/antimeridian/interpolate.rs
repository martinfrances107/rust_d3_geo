use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::InterpolateFn;
use crate::stream::Stream;

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
            let epsilon = T::from(1e-6).unwrap();
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
