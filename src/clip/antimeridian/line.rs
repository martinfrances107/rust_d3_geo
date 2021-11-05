use std::default::Default;

use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::clip::Clean;
use crate::clip::CleanState;
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
    clean: CleanState,
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
            clean: CleanState::NoIntersections,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<T> Clean for Line<T>
where
    T: CoordFloat,
{
    #[inline]
    fn clean(&self) -> CleanState {
        match self.clean {
            // if intersections, rejoin first and last segments
            CleanState::IntersectionsOrEmpty => CleanState::IntersectionsRejoin,
            CleanState::NoIntersections => CleanState::NoIntersections,
            CleanState::IntersectionsRejoin => CleanState::IntersectionsOrEmpty,
        }
    }
}

impl<SINK, T> Stream for StreamNode<Line<T>, SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn line_start(&mut self) {
        self.sink.borrow_mut().line_start();
        self.raw.clean = CleanState::NoIntersections;
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

        let mut s = self.sink.borrow_mut();
        if (delta - T::PI()).abs() < self.raw.epsilon {
            // Line crosses a pole.
            let f_2 = T::from(2_f64).unwrap();
            self.raw.phi0 = if (self.raw.phi0 + phi1 / f_2).is_sign_positive() {
                T::FRAC_PI_2()
            } else {
                -T::FRAC_PI_2()
            };
            s.point(
                &Coordinate {
                    x: self.raw.lambda0,
                    y: self.raw.phi0,
                },
                None,
            );
            s.point(
                &Coordinate {
                    x: self.raw.sign0,
                    y: self.raw.phi0,
                },
                None,
            );
            s.line_end();
            s.line_start();
            s.point(
                &Coordinate {
                    x: sign1,
                    y: self.raw.phi0,
                },
                None,
            );
            s.point(
                &Coordinate {
                    x: lambda1,
                    y: self.raw.phi0,
                },
                None,
            );
            self.raw.clean = CleanState::IntersectionsOrEmpty;
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
            s.point(
                &Coordinate {
                    x: self.raw.sign0,
                    y: self.raw.phi0,
                },
                None,
            );
            s.line_end();
            s.line_start();
            s.point(
                &Coordinate {
                    x: sign1,
                    y: self.raw.phi0,
                },
                None,
            );
            self.raw.clean = CleanState::IntersectionsOrEmpty;
        }

        self.raw.lambda0 = lambda1;
        self.raw.phi0 = phi1;
        s.point(
            &Coordinate {
                x: self.raw.lambda0,
                y: self.raw.phi0,
            },
            None,
        );
        self.raw.sign0 = sign1;
    }

    fn line_end(&mut self) {
        self.sink.borrow_mut().line_end();
        self.raw.lambda0 = T::nan();
        self.raw.phi0 = T::nan();
    }
}
