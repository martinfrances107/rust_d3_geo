use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::path::PathResultEnum;
use crate::stream::Stream;
use crate::stream::StreamClipLine;
// use crate::stream::StreamClipLineNode;
use crate::clip::ClipBuffer;
use crate::stream::StreamClone;
use crate::stream::StreamPathResult;
// use crate::stream::StreamPathResultTrait;
use crate::stream::StreamPathResultNodeStub;

use crate::{
    clip::BufferInTrait,
    stream::{Clean, CleanEnum, StreamClean},
};

use super::intersect::intersect;
pub struct Line<T>
where
    T: CoordFloat + FloatConst,
{
    clean: CleanEnum,
    lambda0: T,
    phi0: T,
    sign0: T,
    stream: Box<dyn StreamPathResult<ScC = Coordinate<T>, Out = Option<PathResultEnum<T>>>>,
}

impl<T> Clone for Line<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.clone_box(),
            ..*self
        }
    }
}
impl<T> Default for Line<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn default() -> Self {
        Line {
            clean: CleanEnum::IntersectionsOrEmpty,
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            stream: Box::new(StreamPathResultNodeStub::default()),
        }
    }
}
impl<T> StreamClipLine for Line<T> where T: CoordFloat + FloatConst + 'static {}
impl<T> BufferInTrait for Line<T>
where
    T: CoordFloat + FloatConst,
{
    // type BitSink = Box<dyn StreamPathResult<Out = Option<PathResultEnum<T>>, ScC = Coordinate<T>>>;
    type BitCB = ClipBuffer<T>;
    #[inline]
    fn buffer_in(&mut self, stream: Self::BitCB) {
        // self.stream = stream;
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

impl<T> StreamClean<T> for Line<T> where T: CoordFloat + FloatConst + 'static {}

impl<T: CoordFloat + FloatConst + 'static> StreamClone for Line<T> {
    type ScC = Coordinate<T>;
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}

impl<T: CoordFloat + FloatConst + 'static> Stream for Line<T> {
    fn line_start(&mut self) {
        // let mut s = self.stream.borrow_mut();
        self.stream.line_start();
        self.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: Coordinate<T>, _m: Option<u8>) {
        let mut lambda1 = p.x;
        let phi1 = p.y;
        // let mut s = self.stream.borrow_mut();
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
                    self.stream.point(
                        Coordinate {
                            x: self.lambda0,
                            y: T::FRAC_PI_2(),
                        },
                        None,
                    );
                }
                false => {
                    self.stream.point(
                        Coordinate {
                            x: self.lambda0,
                            y: -T::FRAC_PI_2(),
                        },
                        None,
                    );
                }
            }
            self.stream.point(
                Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.stream.line_end();
            self.stream.line_start();
            self.stream.point(
                Coordinate {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.stream.point(
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
            self.stream.point(
                Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.stream.line_end();
            //  self.stream.line_start();
            self.stream.point(
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
        self.stream.point(
            Coordinate {
                x: self.lambda0,
                y: self.phi0,
            },
            None,
        );
        self.sign0 = sign1;
    }

    fn line_end(&mut self) {
        // let mut s = self.stream.borrow_mut();
        self.stream.line_end();
        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }
}
