use std::default::Default;
use std::fmt::Debug;

use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::clip::Clean;

use crate::clip::Line as LineTrait;
use crate::math::EPSILON;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

use super::intersect::intersect;

/// Antimeridian Line.
#[derive(Debug, Copy, Clone)]
pub struct Line<T>
where
    T: CoordFloat,
{
    lambda0: T,
    phi0: T,
    sign0: T,
    clean: u8,
    epsilon: T,
}

impl<T> LineTrait for Line<T> where T: CoordFloat {}

impl<T> Default for Line<T>
where
    T: CoordFloat,
{
    fn default() -> Line<T> {
        Self {
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            clean: 0,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<T> Clean for Line<T>
where
    T: CoordFloat,
{
    #[inline]
    fn clean(&self) -> u8 {
        2 - self.clean
    }
}

impl<EP, SINK, T> Stream for StreamNode<EP, Line<T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.sink.get_endpoint()
    }

    fn line_start(&mut self) {
        self.sink.line_start();
        self.raw.clean = 1;
    }

    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        let mut lambda1 = p.x;
        let phi1 = p.y;
        let sign1 = if lambda1 > T::zero() {
            T::PI()
        } else {
            -T::PI()
        };
        let delta = (lambda1 - self.raw.lambda0).abs();

        if (delta - T::PI()).abs() < self.raw.epsilon {
            // Line crosses a pole.
            let f_2 = T::from(2_f64).unwrap();
            self.raw.phi0 = if (self.raw.phi0 + phi1 / f_2).is_sign_positive() {
                T::FRAC_PI_2()
            } else {
                -T::FRAC_PI_2()
            };
            self.sink.point(
                &Coordinate {
                    x: self.raw.lambda0,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.point(
                &Coordinate {
                    x: self.raw.sign0,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.line_end();
            self.sink.line_start();
            self.sink.point(
                &Coordinate {
                    x: sign1,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.point(
                &Coordinate {
                    x: lambda1,
                    y: self.raw.phi0,
                },
                None,
            );
            self.raw.clean = 0;
        } else if self.raw.sign0 != sign1 && delta >= T::PI() {
            // Line crosses antimeridian.
            if (self.raw.lambda0 - self.raw.sign0).abs() < self.raw.epsilon {
                self.raw.lambda0 = self.raw.lambda0 - self.raw.sign0 * self.raw.epsilon;
                // handle degeneracies
            }
            if (lambda1 - sign1).abs() < self.raw.epsilon {
                lambda1 = lambda1 - sign1 * self.raw.epsilon;
            }
            self.raw.phi0 = intersect(self.raw.lambda0, self.raw.phi0, lambda1, phi1);
            self.sink.point(
                &Coordinate {
                    x: self.raw.sign0,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.line_end();
            self.sink.line_start();
            self.sink.point(
                &Coordinate {
                    x: sign1,
                    y: self.raw.phi0,
                },
                None,
            );
            self.raw.clean = 0;
        }

        self.raw.lambda0 = lambda1;
        self.raw.phi0 = phi1;
        self.sink.point(
            &Coordinate {
                x: self.raw.lambda0,
                y: self.raw.phi0,
            },
            None,
        );
        self.raw.sign0 = sign1;
    }

    fn line_end(&mut self) {
        self.sink.line_end();
        self.raw.lambda0 = T::nan();
        self.raw.phi0 = T::nan();
    }
}
