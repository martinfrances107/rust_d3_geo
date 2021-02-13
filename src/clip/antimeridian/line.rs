use std::cell::RefCell;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::stream::StreamClipLine;
use crate::stream::StreamClipLineNode;
use crate::stream::StreamPathResultNodeStub;
use crate::stream::{Stream, StreamPathResultNode};
use crate::{
    clip::BufferInTrait,
    stream::{Clean, CleanEnum, StreamClean},
};

use super::intersect::intersect;
#[derive(Clone)]
pub struct Line<T>
where
    T: CoordFloat + FloatConst,
{
    clean: CleanEnum,
    lambda0: T,
    phi0: T,
    sign0: T,
    stream: StreamPathResultNode<T>,
}

impl<T> Line<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    #[inline]
    pub fn new() -> Self {
        Line {
            clean: CleanEnum::IntersectionsOrEmpty,
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            stream: StreamPathResultNodeStub::new(),
        }
    }

    #[inline]
    pub fn gen_node() -> StreamClipLineNode<T> {
        Rc::new(RefCell::new(Box::new(Line::new())))
    }
}
impl<T> StreamClipLine<T> for Line<T> where T: CoordFloat + FloatConst {}
impl<T> BufferInTrait<T> for Line<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn buffer_in(&mut self, stream: StreamPathResultNode<T>) {
        self.stream = stream;
    }
}

impl<T> Clean for Line<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn clean(&self) -> CleanEnum {
        match self.clean {
            // if intersections, rejoin first and last segments
            CleanEnum::IntersectionsOrEmpty => CleanEnum::IntersectionsRejoin,
            CleanEnum::NoIntersections => CleanEnum::NoIntersections,
            CleanEnum::IntersectionsRejoin => CleanEnum::IntersectionsOrEmpty,
        }
    }
}

impl<T> StreamClean<T> for Line<T> where T: CoordFloat + FloatConst {}
impl<T: CoordFloat + FloatConst> Stream<T> for Line<T> {
    fn line_start(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.line_start();
        self.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: Coordinate<T>, _m: Option<u8>) {
        let mut lambda1 = p.x;
        let phi1 = p.y;
        let mut s = self.stream.borrow_mut();
        let sign1 = match lambda1.is_sign_positive() {
            true => T::PI(),
            false => -T::PI(),
        };
        let delta = (lambda1 - self.lambda0).abs();

        if (delta - T::PI()).abs() < T::epsilon() {
            // Line crosses a pole.
            let f_2 = T::from(2f64).unwrap();
            self.phi0 = (self.phi0 + phi1) / f_2;
            match (self.phi0 + phi1 / f_2).is_sign_positive() {
                true => {
                    s.point(
                        Coordinate {
                            x: self.lambda0,
                            y: T::FRAC_PI_2(),
                        },
                        None,
                    );
                }
                false => {
                    s.point(
                        Coordinate {
                            x: self.lambda0,
                            y: -T::FRAC_PI_2(),
                        },
                        None,
                    );
                }
            }
            s.point(
                Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            s.line_end();
            s.line_start();
            s.point(
                Coordinate {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            s.point(
                Coordinate {
                    x: lambda1,
                    y: self.phi0,
                },
                None,
            );
            self.clean = CleanEnum::IntersectionsOrEmpty;
        } else if self.sign0 != sign1 && delta >= T::PI() {
            // Line crosses antimeridian.
            if (self.lambda0 - self.sign0).abs() < T::epsilon() {
                self.lambda0 = self.lambda0 - self.sign0 * T::epsilon(); // handle degeneracies
            }
            if (lambda1 - sign1).abs() < T::epsilon() {
                lambda1 = lambda1 - sign1 * T::epsilon();
            }
            self.phi0 = intersect(self.lambda0, self.phi0, lambda1, phi1);
            s.point(
                Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            s.line_end();
            //  self.stream.line_start();
            s.point(
                Coordinate {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.clean = CleanEnum::IntersectionsOrEmpty;
        }
        self.lambda0 = lambda1;
        self.phi0 = phi1;
        s.point(
            Coordinate {
                x: self.lambda0,
                y: self.phi0,
            },
            None,
        );
        self.sign0 = sign1;
    }

    fn line_end(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.line_end();
        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }
}
