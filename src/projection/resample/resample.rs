use crate::Transform;
use std::fmt::Display;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

const MAXDEPTH: u8 = 16_u8; // maximum depth of subdivision

#[derive(Clone, Copy, Debug)]
pub struct Resample<PR, T>
where
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
    PR: ProjectionRaw<T = T> + Transform<T = T>,
{
    pub projection_raw: PR,
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
    // pub stream: Box<STREAM>,
    // pub stream: STREAM,
    pub use_line_point: bool,
    pub use_line_start: bool,
    pub use_line_end: bool,
}

impl<'a, PR, T> Resample<PR, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn new(projection_raw: PR) -> Resample<PR, T> {
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
            use_line_point: false,
            use_line_start: false,
            use_line_end: false,
        }
    }
}

impl<'a, PR, SINK, T> StreamNode<Resample<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    SINK: Stream<T = T>,
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]

    fn ring_start(&mut self) {
        self.sink.borrow_mut().line_start();
        self.raw.use_line_point = false;
        self.raw.use_line_end = false;
    }

    fn ring_point(&mut self, p: &Coordinate<T>) {
        self.raw.lambda00 = p.x;
        self.line_point(&Coordinate {
            x: self.raw.lambda00,
            y: p.y,
        });
        self.raw.x00 = self.raw.x0;
        self.raw.y00 = self.raw.y0;
        self.raw.a00 = self.raw.a0;
        self.raw.b00 = self.raw.b0;
        self.raw.c00 = self.raw.c0;
        self.raw.use_line_point = true;
    }

    fn ring_end(&mut self) {
        {
            self.resample_line_to(
                self.raw.x0,
                self.raw.y0,
                self.raw.lambda0,
                self.raw.a0,
                self.raw.b0,
                self.raw.c0,
                self.raw.x00,
                self.raw.y00,
                self.raw.lambda00,
                self.raw.a00,
                self.raw.b00,
                self.raw.c00,
                MAXDEPTH,
            );
        }
        self.raw.use_line_end = true;

        self.sink.borrow_mut().line_end();
    }

    fn line_point(&mut self, p: &Coordinate<T>) {
        let c = cartesian(&p);
        let p_transformed = self.raw.projection_raw.transform(p);
        self.resample_line_to(
            self.raw.x0,
            self.raw.y0,
            self.raw.lambda0,
            self.raw.a0,
            self.raw.b0,
            self.raw.c0,
            p_transformed.x,
            p_transformed.y,
            p.x,
            c[0],
            c[1],
            c[2],
            MAXDEPTH,
        );
        self.raw.x0 = p_transformed.x;
        self.raw.y0 = p_transformed.y;
        self.raw.lambda0 = p.x;
        self.raw.a0 = c[0];
        self.raw.b0 = c[1];
        self.raw.c0 = c[2];
        self.sink.borrow_mut().point(
            &Coordinate {
                x: self.raw.x0,
                y: self.raw.y0,
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
    ) {
        let mut depth = depth_p;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d2 = dx * dx + dy * dy;

        if d2 > T::from(4_f64).unwrap() * self.raw.delta2 {
            depth -= 1_u8;
            if depth > 0_u8 {
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

                let project_ptr = &self.raw.projection_raw;
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
                if dz * dz / d2 > self.raw.delta2
                    || ((dx * dx2 + dy * dy2) / d2 - T::from(0.5).unwrap()).abs()
                        > T::from(0.3).unwrap()
                    || a0 * a1 + b0 * b1 + c0 * c1 < self.raw.cos_min_distance
                {
                    a = a / m;
                    b = b / m;
                    self.resample_line_to(
                        x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b, c, depth,
                    );

                    self.sink
                        .borrow_mut()
                        .point(&Coordinate { x: x2, y: y2 }, None);

                    self.resample_line_to(
                        x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth,
                    );
                }
            }
        }
    }
}

impl<'a, PR, SINK, T> Stream for StreamNode<Resample<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    SINK: Stream<T = T>,
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;

    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere();
    }
    fn polygon_start(&mut self) {
        self.sink.borrow_mut().polygon_start();
    }
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end();
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        if self.raw.use_line_point {
            self.line_point(p);
        } else {
            self.ring_point(p);
        }
    }

    fn line_start(&mut self) {
        if self.raw.use_line_start {
            self.raw.x0 = T::nan();
            self.raw.use_line_point = true;
            self.sink.borrow_mut().line_start();
        } else {
            self.ring_start();
        }
    }

    fn line_end(&mut self) {
        match self.raw.use_line_end {
            true => {
                self.raw.use_line_point = false;
                self.sink.borrow_mut().line_end();
            }

            false => {
                self.ring_end();
            }
        }
    }
}
