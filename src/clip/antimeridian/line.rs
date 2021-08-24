use std::default::Default;

use geo::{CoordFloat, Coordinate};

use num_traits::FloatConst;

use crate::clip::Clean;
use crate::clip::CleanEnum;
use crate::clip::Line as LineTrait;
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
    clean: CleanEnum,
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
            clean: CleanEnum::NoIntersections,
        }
    }
}

// impl<T> LineTrait for Line<T> where
//     // SINK: Stream<T = T>,
//     T: CoordFloat,
// {
// }

impl<T> Clean for Line<T>
where
    T: CoordFloat,
{
    #[inline]
    fn clean(&self) -> CleanEnum {
        // println!("line(A) clean  initial value{:?}", self.clean);
        match self.clean {
            // if intersections, rejoin first and last segments
            CleanEnum::IntersectionsOrEmpty => CleanEnum::IntersectionsRejoin,
            CleanEnum::NoIntersections => CleanEnum::NoIntersections,
            CleanEnum::IntersectionsRejoin => CleanEnum::IntersectionsOrEmpty,
            CleanEnum::Undefined => panic!("Undefined should not be cleaned."),
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
        // println!("line(a) line_start()");
        self.sink.borrow_mut().line_start();
        self.raw.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        // println!("line(a) point {:?} {:?}", p, m);
        let mut lambda1 = p.x;
        let phi1 = p.y;
        let sign1 = if lambda1 > T::zero() {
            T::PI()
        } else {
            -T::PI()
        };
        let delta = (lambda1 - self.raw.lambda0).abs();

        let mut s = self.sink.borrow_mut();
        if (delta - T::PI()).abs() < T::epsilon() {
            // Line crosses a pole.
            println!("line crosses a pole.");
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
            self.raw.clean = CleanEnum::IntersectionsOrEmpty;
        } else if self.raw.sign0 != sign1 && delta >= T::PI() {
            // Line crosses antimeridian.
            println!("line crosses antimeridian.");
            if (self.raw.lambda0 - self.raw.sign0).abs() < T::epsilon() {
                self.raw.lambda0 = self.raw.lambda0 - self.raw.sign0 * T::epsilon();
                // handle degeneracies
            }
            if (lambda1 - sign1).abs() < T::epsilon() {
                lambda1 = lambda1 - sign1 * T::epsilon();
            }
            self.raw.phi0 = intersect(self.raw.lambda0, self.raw.phi0, lambda1, phi1);
            println!("output of intersect {:?}", self.raw.phi0);
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
            self.raw.clean = CleanEnum::IntersectionsOrEmpty;
        } else {
            println!("line crossed nothing");
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
        // println!(" line_end");
        self.sink.borrow_mut().line_end();
        self.raw.lambda0 = T::nan();
        self.raw.phi0 = T::nan();
    }
}
