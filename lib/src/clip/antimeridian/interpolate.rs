// use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

// use crate::clip::InterpolateFn;
use crate::clip::Interpolator;
use crate::math::EPSILON;
use crate::stream::Stream;

/// State for Antimeridian Interpolator.
#[derive(Clone, Debug)]
pub struct Interpolate<T> {
    epsilon: T,
}

impl<T> Default for Interpolate<T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Self {
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}
/// Antimerdian interpolate function.
impl<T> Interpolator for Interpolate<T>
where
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;
    // type EP = EP;
    // type Stream = STREAM;
    fn interpolate<EP, STREAM>(
        &mut self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
        stream_in: &mut STREAM,
    ) where
        STREAM: Stream<EP = EP, T = T>,
    {
        let phi: T;
        match from {
            None => {
                phi = direction * T::FRAC_PI_2();

                stream_in.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coordinate {
                        x: T::zero(),
                        y: phi,
                    },
                    None,
                );
                stream_in.point(&Coordinate { x: T::PI(), y: phi }, None);

                stream_in.point(
                    &Coordinate {
                        x: T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream_in.point(
                    &Coordinate {
                        x: T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coordinate {
                        x: T::zero(),
                        y: -phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream_in.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
            }
            Some(from) => {
                let to = to.unwrap();
                if (from.x - to.x).abs() > self.epsilon {
                    let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                    phi = direction * lambda / T::from(2).unwrap();
                    stream_in.point(&Coordinate { x: -lambda, y: phi }, None);
                    stream_in.point(
                        &Coordinate {
                            x: T::zero(),
                            y: phi,
                        },
                        None,
                    );
                    stream_in.point(&Coordinate { x: lambda, y: phi }, None);
                } else {
                    stream_in.point(&to, None);
                }
            }
        }
    }
}
