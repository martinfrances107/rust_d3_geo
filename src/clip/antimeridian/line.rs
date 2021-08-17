use std::default::Default;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;

use num_traits::FloatConst;

use crate::clip::Clean;
use crate::clip::CleanEnum;
use crate::clip::Line as LineTrait;
use crate::clip::LineRaw;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

use super::intersect::intersect;

/// Antimeridian Line.
#[derive(Debug, Copy, Clone)]
pub struct Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    lambda0: T,
    phi0: T,
    sign0: T,
    clean: CleanEnum,
}

impl<T> LineRaw for Line<T> where T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst {}

impl<T> Default for Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
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

impl<SINK, T> LineTrait for StreamNode<Line<T>, SINK, T>
where
    SINK: Stream<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
}

impl<SINK, T> Clean for StreamNode<Line<T>, SINK, T>
where
    SINK: Stream<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    fn clean(&self) -> CleanEnum {
        println!("line(A) clean  initial value{:?}", self.raw.clean);
        match self.raw.clean {
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
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
    fn sphere(&mut self) {
        todo!("is this called")
    }
    fn polygon_start(&mut self) {
        todo!("is this called")
    }

    fn polygon_end(&mut self) {
        todo!("is this called")
    }

    fn line_start(&mut self) {
        println!("line(a) line_start()");
        self.sink.borrow_mut().line_start();
        self.raw.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("line(a) point {:?} {:?}", p, m);
        let mut lambda1 = p.x;
        let phi1 = p.y;
        let sign1 = if lambda1 > T::zero() {
            T::PI()
        } else {
            -T::PI()
        };
        let delta = (lambda1 - self.raw.lambda0).abs();

        if (delta - T::PI()).abs() < T::epsilon() {
            // Line crosses a pole.
            println!("line crosses a pole.");
            let f_2 = T::from(2_f64).unwrap();
            self.raw.phi0 = if (self.raw.phi0 + phi1 / f_2).is_sign_positive() {
                T::FRAC_PI_2()
            } else {
                -T::FRAC_PI_2()
            };
            self.sink.borrow_mut().point(
                &Coordinate {
                    x: self.raw.lambda0,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.borrow_mut().point(
                &Coordinate {
                    x: self.raw.sign0,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.borrow_mut().line_end();
            self.sink.borrow_mut().line_start();
            self.sink.borrow_mut().point(
                &Coordinate {
                    x: sign1,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.borrow_mut().point(
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
            self.sink.borrow_mut().point(
                &Coordinate {
                    x: self.raw.sign0,
                    y: self.raw.phi0,
                },
                None,
            );
            self.sink.borrow_mut().line_end();
            self.sink.borrow_mut().line_start();
            self.sink.borrow_mut().point(
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
        self.sink.borrow_mut().point(
            &Coordinate {
                x: self.raw.lambda0,
                y: self.raw.phi0,
            },
            None,
        );
        self.raw.sign0 = sign1;
    }

    fn line_end(&mut self) {
        println!("line(a) line_end");
        self.sink.borrow_mut().line_end();
        self.raw.lambda0 = T::nan();
        self.raw.phi0 = T::nan();
    }
}
