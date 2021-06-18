use std::fmt::Display;
// use std::marker::PhantomData;
use std::ops::AddAssign;
// use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

// use crate::clip::line_sink_enum::LineSinkEnum;
// use crate::clip::ClipBuffer;
// use crate::projection::projection_trait::ProjectionTrait;s
// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
use crate::clip::clip_buffer::ClipBuffer;
use crate::clip::Clean;
use crate::clip::CleanEnum;
use crate::clip::LCB;
use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;

// use crate::Transform;

use super::intersect::intersect;

#[derive(Debug, Default)]
pub struct Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    // SD: StreamDst,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + Float + FloatConst,
{
    // pd: PhantomData<&'a u8>,
    clean: CleanEnum,
    lambda0: T,
    phi0: T,
    sign0: T,
    stream: STREAM,
}

// impl<'a, STREAM, T> Default for Line<'a, STREAM, T>
// where
//     // Rc<PR>: Transform<C = Coordinate<T>>,
//     // PR: Transform<C = Coordinate<T>>,
//     STREAM: Stream<SC=Coordinate<T>> + Default,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     #[inline]
//     fn default() -> Self {
//         Line {
//             pd: PhantomData,
//             clean: CleanEnum::Undefined,
//             lambda0: T::nan(),
//             phi0: T::nan(),
//             sign0: T::nan(),
//             stream: STREAM::default()
//         }
//     }
// }

impl<T> LCB for Line<ClipBuffer<T>, T> where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst
{
}

impl<STREAM, T> StreamIn for Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SInput = STREAM;
    #[inline]
    fn stream_in(&mut self, stream: STREAM) {
        self.stream = stream;
    }

    // #[inline]
    // fn get_stream(&'a mut self) -> &'a mut STREAM {
    //     &mut self.stream
    // }
}

impl<STREAM, T> Clean for Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    fn clean(&self) -> CleanEnum {
        println!("line(A) clean  initial value{:?}", self.clean);
        match self.clean {
            // if intersections, rejoin first and last segments
            CleanEnum::IntersectionsOrEmpty => CleanEnum::IntersectionsRejoin,
            CleanEnum::NoIntersections => CleanEnum::NoIntersections,
            CleanEnum::IntersectionsRejoin => CleanEnum::IntersectionsOrEmpty,
            CleanEnum::Undefined => panic!("Undefined should not be cleaned."),
        }
    }
}

impl<STREAM, T> Stream for Line<STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    // PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SC = Coordinate<T>;
    // type SD = SD;
    // type ST = T;
    fn sphere(&mut self) {
        todo!("is this called")
    }
    fn polygon_start(&mut self) {
        todo!("is this called")
    }

    fn polygon_end(&mut self) {
        todo!("is this called")
    }

    // fn get_dst(
    //     &self,
    // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
    // {
    //     self.stream.get_dst()
    // }

    fn line_start(&mut self) {
        println!("line(a) line_start()");
        self.stream.line_start();
        self.clean = CleanEnum::NoIntersections;
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
        let delta = (lambda1 - self.lambda0).abs();

        if (delta - T::PI()).abs() < T::epsilon() {
            // Line crosses a pole.
            println!("line crosses a pole.");
            let f_2 = T::from(2f64).unwrap();
            self.phi0 = if (self.phi0 + phi1 / f_2).is_sign_positive() {
                T::FRAC_PI_2()
            } else {
                -T::FRAC_PI_2()
            };
            self.stream.point(
                &Coordinate {
                    x: self.lambda0,
                    y: self.phi0,
                },
                None,
            );
            self.stream.point(
                &Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.stream.line_end();
            self.stream.line_start();
            self.stream.point(
                &Coordinate {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.stream.point(
                &Coordinate {
                    x: lambda1,
                    y: self.phi0,
                },
                None,
            );
            self.clean = CleanEnum::IntersectionsOrEmpty;
        } else if self.sign0 != sign1 && delta >= T::PI() {
            // Line crosses antimeridian.
            println!("line crosses antimeridian.");
            if (self.lambda0 - self.sign0).abs() < T::epsilon() {
                self.lambda0 = self.lambda0 - self.sign0 * T::epsilon(); // handle degeneracies
            }
            if (lambda1 - sign1).abs() < T::epsilon() {
                lambda1 = lambda1 - sign1 * T::epsilon();
            }
            self.phi0 = intersect(self.lambda0, self.phi0, lambda1, phi1);
            println!("output of intersect {:?}", self.phi0);
            self.stream.point(
                &Coordinate {
                    x: self.sign0,
                    y: self.phi0,
                },
                None,
            );
            self.stream.line_end();
            self.stream.line_start();
            self.stream.point(
                &Coordinate {
                    x: sign1,
                    y: self.phi0,
                },
                None,
            );
            self.clean = CleanEnum::IntersectionsOrEmpty;
        } else {
            println!("line crossed nothing");
        }

        self.lambda0 = lambda1;
        self.phi0 = phi1;
        self.stream.point(
            &Coordinate {
                x: self.lambda0,
                y: self.phi0,
            },
            None,
        );
        self.sign0 = sign1;
    }

    fn line_end(&mut self) {
        println!("line(a) line_end");
        self.stream.line_end();
        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }
}
