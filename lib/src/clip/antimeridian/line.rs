// use crate::projection::builder::template::ResampleNoneNoClipC;
// use crate::Transform;
use std::default::Default;
use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::LineConnected;
use crate::clip::LineUnconnected;
use crate::math::EPSILON;
// use crate::projection::builder::template::ResampleNoneNoClipC;
// use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::ConnectedState;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::intersect::intersect;

/// Antimeridian Line.
#[derive(Debug, Copy, Clone)]
pub struct Line<EP, SC, STATE, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    T: CoordFloat,
{
    state: STATE,
    p_ep: PhantomData<EP>,
    p_sc: PhantomData<SC>,
    lambda0: T,
    phi0: T,
    sign0: T,
    clean: u8,
    epsilon: T,
}
// Note Default is ONLY implenented for the unconnected state
// Added when I found it was useful for type corercion.
impl<EP, RC, T> Default for Line<EP, RC, Unconnected, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    // PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Line<EP, RC, Unconnected, T> {
        Self {
            state: Unconnected,
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<RC>,
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            clean: 0,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<EP, SC, T> Bufferable for Line<EP, SC, Unconnected, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    EP: Clone + Debug,
    T: CoordFloat,
{
    type T = T;
    type Output = Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>;
    fn buffer(self, buffer: Buffer<T>) -> Self::Output {
        Line {
            state: Connected { sink: buffer },
            p_ep: PhantomData::<Buffer<T>>,
            p_sc: PhantomData::<Buffer<T>>,
            lambda0: self.lambda0,
            phi0: self.phi0,
            sign0: self.sign0,
            clean: self.clean,
            epsilon: self.epsilon,
        }
    }
}

impl<EP, SC, T> Connectable for Line<EP, SC, Unconnected, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    T: CoordFloat,
{
    type Output = Line<EP, SC, Connected<SC>, T>;
    type SC = SC;

    fn connect(self, sink: SC) -> Line<EP, SC, Connected<SC>, T> {
        Line {
            state: Connected { sink },
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<SC>,
            lambda0: self.lambda0,
            phi0: self.phi0,
            sign0: self.sign0,
            clean: self.clean,
            epsilon: self.epsilon,
        }
    }
}

impl<EP, SINK, T> LineUnconnected for Line<EP, SINK, Unconnected, T>
where
    EP: Clone + Debug,
    SINK: Clone + Debug,
    // EP: Stream<EP = EP, T = T> + Default,
    // SINK: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    type SU = SINK;
}

impl<EP, SINK, T> LineConnected for Line<EP, SINK, Connected<SINK>, T>
where
    EP: Clone + Debug,
    SINK: Clone + Debug,
    // EP: Stream<EP = EP, T = T> + Default,
    // SINK: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    type SC = SINK;
    #[inline]
    fn get_sink(&mut self) -> &mut SINK {
        &mut self.state.sink
    }
}

impl<EP, SINK, T> Clean for Line<EP, SINK, Connected<SINK>, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    T: CoordFloat,
{
    #[inline]
    fn clean(&self) -> u8 {
        2 - self.clean
    }
}

impl<EP, SINK, T> Stream for Line<EP, SINK, Connected<SINK>, T>
where
    EP: Stream<EP = EP, T = T> + Default,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(&mut self) -> &mut Self::EP {
        self.state.get_sink().get_endpoint()
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
        // let state = self.state;
        // let sink = self.state.get_sink();
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
