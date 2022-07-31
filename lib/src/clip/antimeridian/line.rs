use std::default::Default;
use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
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
pub struct Line<SC, STATE, T> {
    state: STATE,
    /// PhantomData<SC>
    ///
    /// The hidden linkage in Connectable::connect.
    /// Changing the input paramter changes the output
    /// parameter.
    p_sc: PhantomData<SC>,
    lambda0: T,
    phi0: T,
    sign0: T,
    clean: u8,
    epsilon: T,
}
// Note Default is ONLY implenented for the unconnected state
// Added when I found it was useful for type corercion.
impl<RC, T> Default for Line<RC, Unconnected, T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Line<RC, Unconnected, T> {
        Self {
            state: Unconnected,
            p_sc: PhantomData::<RC>,
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            clean: 0,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<SC, T> Bufferable for Line<SC, Unconnected, T>
where
    T: CoordFloat,
{
    type Output = Line<Buffer<T>, Connected<Buffer<T>>, T>;
    type T = T;

    fn buffer(self, buffer: Buffer<T>) -> Self::Output {
        Line {
            state: Connected { sink: buffer },
            p_sc: PhantomData::<Buffer<T>>,
            lambda0: self.lambda0,
            phi0: self.phi0,
            sign0: self.sign0,
            clean: self.clean,
            epsilon: self.epsilon,
        }
    }
}

impl<SC, T> Connectable for Line<SC, Unconnected, T>
where
    T: CoordFloat,
{
    type Output = Line<SC, Connected<SC>, T>;
    type SC = SC;

    fn connect(self, sink: SC) -> Line<SC, Connected<SC>, T> {
        Line {
            state: Connected { sink },
            p_sc: PhantomData::<SC>,
            lambda0: self.lambda0,
            phi0: self.phi0,
            sign0: self.sign0,
            clean: self.clean,
            epsilon: self.epsilon,
        }
    }
}

impl<SINK, T> LineConnected for Line<SINK, Connected<SINK>, T>
where
    T: CoordFloat,
{
    type SC = SINK;
    #[inline]
    fn sink(&mut self) -> &mut SINK {
        &mut self.state.sink
    }
}

impl<SINK, T> Clean for Line<SINK, Connected<SINK>, T>
where
    T: CoordFloat,
{
    #[inline]
    fn clean(&self) -> u8 {
        2 - self.clean
    }
}

impl<EP, SINK, T> Stream for Line<SINK, Connected<SINK>, T>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.get_sink().endpoint()
    }

    fn line_start(&mut self) {
        self.state.get_sink().line_start();
        self.clean = 1;
    }

    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
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
            self.state.get_sink().point(
                &Coordinate {
                    x: self.lambda0,
                    y: self.phi0,
                },
                None,
            );
            self.state.get_sink().point(
                &Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.state.get_sink().line_end();
            self.state.get_sink().line_start();
            self.state.get_sink().point(
                &Coordinate {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.state.get_sink().point(
                &Coordinate {
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
            self.state.get_sink().point(
                &Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.state.get_sink().line_end();
            self.state.get_sink().line_start();
            self.state.get_sink().point(
                &Coordinate {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.clean = 0;
        }
        self.lambda0 = lambda1;
        self.phi0 = phi1;
        self.state.get_sink().point(
            &Coordinate {
                x: self.lambda0,
                y: self.phi0,
            },
            None,
        );
        self.sign0 = sign1;
    }

    fn line_end(&mut self) {
        self.state.get_sink().line_end();
        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }
}
