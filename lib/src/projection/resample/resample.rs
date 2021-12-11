use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::compose::Compose;
use crate::math::EPSILON;
use crate::projection::str::scale_translate_rotate::ScaleTranslateRotate;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::Transform;

static MAXDEPTH: u8 = 16_u8; // maximum depth of subdivision

#[derive(Clone, Copy, Debug)]
enum PointState {
    Default,
    Line,
    Ring,
}

#[derive(Clone, Copy, Debug)]
pub struct Resample<PR, T>
where
    T: CoordFloat + FloatConst,
    PR: ProjectionRaw<T>,
{
    pub projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
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

    point_state: PointState,
    use_line_start: bool,
    use_line_end: bool,

    // generic constants<T>
    epsilon: T,
    four: T,
    frac_1_2: T,
    frac_1_3: T,
    two: T,
}

impl<'a, PR, T> Resample<PR, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat + FloatConst,
{
    pub fn new(
        projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
        delta2: T,
    ) -> Resample<PR, T> {
        Self {
            projection_transform,
            delta2,

            // first point
            lambda00: T::nan(),
            x00: T::nan(),
            y00: T::nan(),
            a00: T::nan(),
            b00: T::nan(),
            c00: T::nan(),

            // previous point
            lambda0: T::nan(),
            x0: T::nan(),
            y0: T::nan(),
            a0: T::nan(),
            b0: T::nan(),
            c0: T::nan(),

            // cos(minimium angular distance)
            cos_min_distance: T::from(30_f64).unwrap().to_radians().cos(),
            point_state: PointState::Default,
            use_line_start: true,
            use_line_end: true,

            // Generic constants.
            epsilon: T::from(EPSILON).unwrap(),
            four: T::from(4_f64).unwrap(),
            frac_1_2: T::from(0.5_f64).unwrap(),
            frac_1_3: T::from(1_f64 / 3_f64).unwrap(),
            two: T::from(2_f64).unwrap(),
        }
    }
}

impl<'a, EP, PR, SINK, T> StreamNode<EP, Resample<PR, T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    PR: ProjectionRaw<T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let pt = self.raw.projection_transform.transform(p);
        self.sink.point(&pt, m);
    }

    fn line_start_default(&mut self) {
        self.raw.x0 = T::nan();
        self.raw.point_state = PointState::Line;
        self.sink.line_start();
    }

    fn line_end_default(&mut self) {
        self.raw.point_state = PointState::Default;
        self.sink.line_end();
    }

    fn ring_start(&mut self) {
        self.sink.line_start();
        self.raw.point_state = PointState::Ring;
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
        self.raw.point_state = PointState::Line;
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

        self.sink.line_end();
    }

    fn line_point(&mut self, p: &Coordinate<T>) {
        let c = cartesian(p);
        let p_transformed = self.raw.projection_transform.transform(p);
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
        self.sink.point(
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

        if d2 > self.raw.four * self.raw.delta2 {
            depth -= 1_u8;
            if depth > 0_u8 {
                let mut a = a0 + a1;
                let mut b = b0 + b1;
                let mut c = c0 + c1;
                let m = (a * a + b * b + c * c).sqrt();
                c = c / m;
                let phi2 = c.asin();
                let lambda2 = if (c.abs() - T::one()).abs() < self.raw.epsilon
                    || (lambda0 - lambda1).abs() < self.raw.epsilon
                {
                    (lambda0 + lambda1) * self.raw.frac_1_2
                } else {
                    b.atan2(a)
                };

                let p = self.raw.projection_transform.transform(&Coordinate {
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
                    || ((dx * dx2 + dy * dy2) / d2 - self.raw.frac_1_2).abs() > self.raw.frac_1_3
                    || a0 * a1 + b0 * b1 + c0 * c1 < self.raw.cos_min_distance
                {
                    a = a / m;
                    b = b / m;
                    self.resample_line_to(
                        x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b, c, depth,
                    );

                    self.sink.point(&Coordinate { x: x2, y: y2 }, None);

                    self.resample_line_to(
                        x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth,
                    );
                }
            }
        }
    }
}

impl<'a, EP, PR, SINK, T> Stream for StreamNode<EP, Resample<PR, T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    PR: ProjectionRaw<T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.sink.get_endpoint()
    }
    fn sphere(&mut self) {
        self.sink.sphere();
    }
    fn polygon_start(&mut self) {
        self.sink.polygon_start();
        self.raw.use_line_start = false;
    }
    fn polygon_end(&mut self) {
        self.sink.polygon_end();
        self.raw.use_line_start = true;
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.raw.point_state {
            PointState::Default => {
                self.point_default(p, m);
            }
            PointState::Line => {
                self.line_point(p);
            }
            PointState::Ring => {
                self.ring_point(p);
            }
        }
    }

    #[inline]
    fn line_start(&mut self) {
        if self.raw.use_line_start {
            self.line_start_default();
        } else {
            self.ring_start();
        }
    }

    #[inline]
    fn line_end(&mut self) {
        if self.raw.use_line_end {
            self.line_end_default();
        } else {
            self.ring_end();
        }
    }
}
