use std::default::Default;
use std::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::LineConnected;
use crate::math::EPSILON;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::ConnectedState;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::intersect::intersect;

/// Antimeridian Line.
#[derive(Debug, Copy, Clone)]
pub struct Line<STATE, T> {
    state: STATE,
    lambda0: T,
    phi0: T,
    sign0: T,
    clean: u8,
    epsilon: T,
}
// Note Default is ONLY implenented for the unconnected state
// Added when I found it was useful for type corercion.
impl<T> Default for Line<Unconnected, T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            state: Unconnected,
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            clean: 0,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<T> Bufferable for Line<Unconnected, T>
where
    T: CoordFloat,
{
    /// The resultant line buffer type.
    type Output = Line<Connected<Buffer<T>>, T>;
    type T = T;

    fn buffer(&mut self, buffer: Buffer<T>) -> Self::Output {
        Line {
            state: Connected { sink: buffer },
            lambda0: self.lambda0,
            phi0: self.phi0,
            sign0: self.sign0,
            clean: self.clean,
            epsilon: self.epsilon,
        }
    }
}

impl<T> Connectable for Line<Unconnected, T>
where
    T: CoordFloat,
{
    /// The resultant line type.
    type Output<SC: Clone> = Line<Connected<SC>, T>;

    fn connect<SC: Clone>(&self, sink: SC) -> Self::Output<SC> {
        Line {
            state: Connected { sink },
            lambda0: self.lambda0,
            phi0: self.phi0,
            sign0: self.sign0,
            clean: self.clean,
            epsilon: self.epsilon,
        }
    }
}

impl<SINK, T> LineConnected for Line<Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    type SC = SINK;
    #[inline]
    fn sink(&mut self) -> &mut SINK {
        &mut self.state.sink
    }
}

impl<SINK, T> Clean for Line<Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    #[inline]
    fn clean(&self) -> u8 {
        2 - self.clean
    }
}

impl<EP, SINK, T> Stream for Line<Connected<SINK>, T>
where
    SINK: Clone + Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink().endpoint()
    }

    fn line_end(&mut self) {
        self.state.sink().line_end();
        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }

    fn line_start(&mut self) {
        self.state.sink().line_start();
        self.clean = 1;
    }

    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        let mut lambda1 = p.x;
        let phi1 = p.y;
        let sign1 = if lambda1 > T::zero() {
            T::PI()
        } else {
            -T::PI()
        };
        let delta = (lambda1 - self.lambda0).abs();
        if (delta - T::PI()).abs() < self.epsilon {
            // Line crosses a pole.
            let f_2 = T::from(2_f64).unwrap();
            self.phi0 = if (self.phi0 + phi1 / f_2).is_sign_positive() {
                T::FRAC_PI_2()
            } else {
                -T::FRAC_PI_2()
            };
            self.state.sink().point(
                &Coord {
                    x: self.lambda0,
                    y: self.phi0,
                },
                None,
            );
            self.state.sink().point(
                &Coord {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.state.sink().line_end();
            self.state.sink().line_start();
            self.state.sink().point(
                &Coord {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.state.sink().point(
                &Coord {
                    x: lambda1,
                    y: self.phi0,
                },
                None,
            );
            self.clean = 0;
        } else if self.sign0 != sign1 && delta >= T::PI() {
            // Line crosses antimeridian.
            if (self.lambda0 - self.sign0).abs() < self.epsilon {
                self.lambda0 = self.lambda0 - self.sign0 * self.epsilon;
                // handle degeneracies
            }
            if (lambda1 - sign1).abs() < self.epsilon {
                lambda1 = lambda1 - sign1 * self.epsilon;
            }
            self.phi0 = intersect(self.lambda0, self.phi0, lambda1, phi1);
            self.state.sink().point(
                &Coord {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.state.sink().line_end();
            self.state.sink().line_start();
            self.state.sink().point(
                &Coord {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.clean = 0;
        }
        self.lambda0 = lambda1;
        self.phi0 = phi1;
        self.state.sink().point(
            &Coord {
                x: self.lambda0,
                y: self.phi0,
            },
            None,
        );
        self.sign0 = sign1;
    }
}
