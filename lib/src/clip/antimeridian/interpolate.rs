use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::Interpolator;
use crate::math::EPSILON;
use crate::stream::Stream;

/// State for Antimeridian Interpolator.
#[derive(Clone, Debug)]
pub struct Interpolate<T> {
    epsilon: T,
    two: T,
}

impl<T> Default for Interpolate<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            epsilon: T::from(EPSILON).unwrap(),
            two: T::from(2).unwrap(),
        }
    }
}
/// Antimerdian interpolate function.
impl<T> Interpolator for Interpolate<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn interpolate<EP, STREAM>(
        &self,
        from: Option<Coord<T>>,
        to: Option<Coord<T>>,
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
                    &Coord {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coord {
                        x: T::zero(),
                        y: phi,
                    },
                    None,
                );
                stream_in.point(&Coord { x: T::PI(), y: phi }, None);

                stream_in.point(
                    &Coord {
                        x: T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream_in.point(
                    &Coord {
                        x: T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coord {
                        x: T::zero(),
                        y: -phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coord {
                        x: -T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream_in.point(
                    &Coord {
                        x: -T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream_in.point(
                    &Coord {
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

                    phi = direction * lambda / self.two;
                    stream_in.point(&Coord { x: -lambda, y: phi }, None);
                    stream_in.point(
                        &Coord {
                            x: T::zero(),
                            y: phi,
                        },
                        None,
                    );
                    stream_in.point(&Coord { x: lambda, y: phi }, None);
                } else {
                    stream_in.point(&to, None);
                }
            }
        }
    }
}
