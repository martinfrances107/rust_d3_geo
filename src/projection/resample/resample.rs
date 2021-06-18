use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
// use crate::clip::clip_sink_enum::ClipSinkEnum;
// use crate::projection::ProjectionRawTrait;
// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::Transform;

use super::ResampleTrait;

const MAXDEPTH: u8 = 16u8; // maximum depth of subdivision

// #[derive(Debug)]
pub struct Resample<'a, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
    STREAM: Stream<SC = Coordinate<T>>,
{
    pub projection_raw: &'a PR,
    pub delta2: T,

    // first point
    pub lambda00: T,
    pub x00: T,
    pub y00: T,
    pub a00: T,
    pub b00: T,
    pub c00: T,

    // previous point
    pub lambda0: T,
    pub x0: T,
    pub y0: T,
    pub a0: T,
    pub b0: T,
    pub c0: T,

    pub cos_min_distance: T,

    // Box here prevents recurson.
    pub stream: Box<STREAM>,
    pub use_line_point: bool,
    pub use_line_start: bool,
    pub use_line_end: bool,
}

// impl<P, T> Clone for Resample<P, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     fn clone(&self) -> Self {
//         Self {
//             project: self.project.clone(),
//             stream: self.stream.clone(),
//             ..*self
//         }
//     }
// }

impl<'a, PR, STREAM, T> Resample<'a, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub fn new(projection_raw: &'a PR) -> Resample<PR, STREAM, T> {
        Self {
            projection_raw,
            delta2: T::zero(),

            // first point
            lambda00: T::zero(),
            x00: T::zero(),
            y00: T::zero(),
            a00: T::zero(),
            b00: T::zero(),
            c00: T::zero(),

            // previous point
            lambda0: T::zero(),
            x0: T::zero(),
            y0: T::zero(),
            a0: T::zero(),
            b0: T::zero(),
            c0: T::zero(),

            cos_min_distance: T::zero(),
            stream: Box::new(STREAM::default()), // stub value

            use_line_point: false,
            use_line_start: false,
            use_line_end: false,
        }
    }
}

impl<PR, STREAM, T> ResampleTrait for Resample<'_, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
}

use crate::stream::stream_in_trait::StreamIn;
impl<'a, PR, STREAM, T> StreamIn for Resample<'a, PR, STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    PR: Transform<C = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
    STREAM: Stream<SC = Coordinate<T>>,
{
    type SInput = STREAM;
    #[inline]
    fn stream_in(&mut self, stream: STREAM) {
        self.stream = Box::new(stream);
    }

    // #[inline]
    // pub fn get_dst(&self) -> dyn StreamDst<SC = Coordinate<T>, SD = SD, T = T, ST = T, Out = SD> {
    //     self.stream.get_dst()
    // }
}

impl<'a, PR, STREAM, T> Resample<'a, PR, STREAM, T>
where
    // Rc<PR>: Transform<C = Coordinate<T>>,
    PR: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]

    fn ring_start(&mut self) {
        self.line_start();
        self.use_line_point = false;
        self.use_line_end = false;
    }

    fn ring_point(&mut self, p: &Coordinate<T>) {
        self.lambda00 = p.x;
        self.line_point(&Coordinate {
            x: self.lambda00,
            y: p.y,
        });
        self.x00 = self.x0;
        self.y00 = self.y0;
        self.a00 = self.a0;
        self.b00 = self.b0;
        self.c00 = self.c0;
        self.use_line_point = true;
    }

    fn ring_end(&mut self) {
        {
            let mut s = self.stream;
            self.resample_line_to(
                self.x0,
                self.y0,
                self.lambda0,
                self.a0,
                self.b0,
                self.c0,
                self.x00,
                self.y00,
                self.lambda00,
                self.a00,
                self.b00,
                self.c00,
                MAXDEPTH,
                &mut s,
            );
        }
        self.use_line_end = true;

        self.stream.line_end();
    }

    fn line_point(&mut self, p: &Coordinate<T>) {
        let c = cartesian(&p);
        let p_transformed = self.projection_raw.transform(&p);
        self.resample_line_to(
            self.x0,
            self.y0,
            self.lambda0,
            self.a0,
            self.b0,
            self.c0,
            p_transformed.x,
            p_transformed.y,
            p.x,
            c[0],
            c[1],
            c[2],
            MAXDEPTH,
            &mut self.stream,
        );
        self.x0 = p_transformed.x;
        self.y0 = p_transformed.y;
        self.lambda0 = p.x;
        self.a0 = c[0];
        self.b0 = c[1];
        self.c0 = c[2];
        self.stream.point(
            &Coordinate {
                x: self.x0,
                y: self.y0,
            },
            None,
        );
    }

    #[allow(clippy::many_single_char_names)]
    fn resample_line_to(
        &mut self,
        x0: T,
        y0: T,
        lambda0: T,
        a0: T,
        b0: T,
        c0: T,
        x1: T,
        y1: T,
        lambda1: T,
        a1: T,
        b1: T,
        c1: T,
        depth_p: u8,
        stream: &mut Box<STREAM>,
    ) {
        let mut depth = depth_p;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d2 = dx * dx + dy * dy;

        if d2 > T::from(4f64).unwrap() * self.delta2 {
            depth -= 1u8;
            if depth > 0u8 {
                let mut a = a0 + a1;
                let mut b = b0 + b1;
                let mut c = c0 + c1;
                let m = (a * a + b * b + c * c).sqrt();
                c = c / m;
                let phi2 = c.asin();
                let lambda2;
                if (c.abs() - T::one()).abs() < T::epsilon()
                    || (lambda0 - lambda1).abs() < T::epsilon()
                {
                    lambda2 = (lambda0 + lambda1) / T::from(2).unwrap();
                } else {
                    lambda2 = b.atan2(a);
                };

                let project_ptr = &self.projection_raw;
                let project = project_ptr;
                let p = project.transform(&Coordinate {
                    x: lambda2,
                    y: phi2,
                });

                let x2 = p.x;
                let y2 = p.y;
                let dx2 = x2 - x0;
                let dy2 = y2 - y0;
                let dz = dy * dx2 - dx * dy2;
                // Three condtions :-
                // perpendicular projected distance
                // midpoint close to an end
                // angular distance
                if dz * dz / d2 > self.delta2
                    || ((dx * dx2 + dy * dy2) / d2 - T::from(0.5).unwrap()).abs()
                        > T::from(0.3).unwrap()
                    || a0 * a1 + b0 * b1 + c0 * c1 < self.cos_min_distance
                {
                    a = a / m;
                    b = b / m;
                    self.resample_line_to(
                        x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b, c, depth, stream,
                    );

                    stream.point(&Coordinate { x: x2, y: y2 }, None);

                    self.resample_line_to(
                        x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth, stream,
                    );
                }
            }
        }
    }
}

impl<'a, PR, STREAM, T> Stream for Resample<'a, PR, STREAM, T>
where
    PR: Transform<C = Coordinate<T>>,
    // Rc<PR>: Transform<C = Coordinate<T>>,
    STREAM: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SC = Coordinate<T>;

    // fn get_dst(
    //     &self,
    // ) -> dyn StreamDst<SC = Self::SC, SD = Self::SD, T = Self::ST, ST = Self::ST, Out = Self::SD>
    // {
    //     self.stream.get_dst()
    // }
    fn sphere(&mut self) {
        self.stream.sphere();
    }
    fn polygon_start(&mut self) {
        self.stream.polygon_start();
    }
    fn polygon_end(&mut self) {
        self.stream.polygon_end();
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        if self.use_line_point {
            self.line_point(p);
        } else {
            self.ring_point(p);
        }
    }

    fn line_start(&mut self) {
        if self.use_line_start {
            self.x0 = T::nan();
            self.use_line_point = true;
            self.stream.line_start();
        } else {
            self.ring_start();
        }
    }

    fn line_end(&mut self) {
        match self.use_line_end {
            true => {
                self.use_line_point = false;
                self.stream.line_end();
            }

            false => {
                self.ring_end();
            }
        }
    }
}
