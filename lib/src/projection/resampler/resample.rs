use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::compose::Compose;
use crate::math::EPSILON;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::stream::Connectable;
use crate::stream::ConnectedState;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

static MAXDEPTH: u8 = 16_u8; // maximum depth of subdivision

#[derive(Clone, Copy, Debug)]
enum PointState {
    Default,
    Line,
    Ring,
}

/// State specific to the resample connection.
#[derive(Clone, Debug)]
pub struct Connected<SINK, T> {
    sink: SINK,
    // first point
    lambda00: T,
    x00: T,
    y00: T,
    a00: T,
    b00: T,
    c00: T,

    // previous point
    lambda0: T,
    x0: T,
    y0: T,
    a0: T,
    b0: T,
    c0: T,

    cos_min_distance: T,

    point_state: PointState,
    use_line_start: bool,
    use_line_end: bool,

    /// generic constants<T>
    epsilon: T,
    four: T,
    frac_1_2: T,
    frac_1_3: T,
}

impl<SINK, T> ConnectedState for Connected<SINK, T>
where
    T: Clone + Debug,
    SINK: Clone + Debug,
{
    type Sink = SINK;

    #[inline]
    fn sink(&mut self) -> &mut Self::Sink {
        &mut self.sink
    }
}

/// Resample the stream base on a given precision.
#[derive(Clone)]
pub struct Resample<PR, SC, STATE, T>
where
    T: CoordFloat,
{
    delta2: T,
    p_sc: PhantomData<SC>,
    projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
    state: STATE,
}

impl<PR, SC, STATE, T> Debug for Resample<PR, SC, STATE, T>
where
    STATE: Debug,
    T: CoordFloat,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.state).finish()
    }
}

impl<PR, SC, T> Connectable for Resample<PR, SC, Unconnected, T>
where
    T: CoordFloat + FloatConst,
{
    type Output = Resample<PR, SC, Connected<SC, T>, T>;
    type SC = SC;

    #[inline]
    fn connect(self, sink: SC) -> Resample<PR, SC, Connected<SC, T>, T> {
        Resample {
            delta2: self.delta2,
            p_sc: self.p_sc,
            projection_transform: self.projection_transform,
            state: Connected {
                sink,
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
            },
        }
    }
}

impl<PR, SC, T> Resample<PR, SC, Unconnected, T>
where
    T: CoordFloat,
{
    /// Returns a Resample for a given precision.
    #[inline]
    pub fn new(
        projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
        delta2: T,
    ) -> Resample<PR, SC, Unconnected, T> {
        Self {
            delta2,
            p_sc: PhantomData::<SC>,
            projection_transform,
            state: Unconnected,
        }
    }
}

impl<EP, PR, SC, T> Resample<PR, SC, Connected<SC, T>, T>
where
    PR: Transform<T = T>,
    SC: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let pt = self.projection_transform.transform(p);
        self.state.sink.point(&pt, m);
    }

    fn line_start_default(&mut self) {
        self.state.x0 = T::nan();
        self.state.point_state = PointState::Line;
        self.state.sink.line_start();
    }

    fn line_end_default(&mut self) {
        self.state.point_state = PointState::Default;
        self.state.sink.line_end();
    }

    fn ring_start(&mut self) {
        self.line_start_default();
        self.state.point_state = PointState::Ring;
        self.state.use_line_end = false;
    }

    fn ring_point(&mut self, p: &Coordinate<T>) {
        self.state.lambda00 = p.x;
        self.line_point(&Coordinate {
            x: self.state.lambda00,
            y: p.y,
        });
        self.state.x00 = self.state.x0;
        self.state.y00 = self.state.y0;
        self.state.a00 = self.state.a0;
        self.state.b00 = self.state.b0;
        self.state.c00 = self.state.c0;
        self.state.point_state = PointState::Line;
    }

    fn ring_end(&mut self) {
        self.resample_line_to(
            self.state.x0,
            self.state.y0,
            self.state.lambda0,
            self.state.a0,
            self.state.b0,
            self.state.c0,
            self.state.x00,
            self.state.y00,
            self.state.lambda00,
            self.state.a00,
            self.state.b00,
            self.state.c00,
            MAXDEPTH,
        );

        self.state.use_line_end = true;

        self.state.sink.line_end();
    }

    fn line_point(&mut self, p: &Coordinate<T>) {
        let c = cartesian(p);
        let p_transformed = self.projection_transform.transform(p);
        self.resample_line_to(
            self.state.x0,
            self.state.y0,
            self.state.lambda0,
            self.state.a0,
            self.state.b0,
            self.state.c0,
            p_transformed.x,
            p_transformed.y,
            p.x,
            c[0],
            c[1],
            c[2],
            MAXDEPTH,
        );
        self.state.x0 = p_transformed.x;
        self.state.y0 = p_transformed.y;
        self.state.lambda0 = p.x;
        self.state.a0 = c[0];
        self.state.b0 = c[1];
        self.state.c0 = c[2];
        self.state.sink.point(
            &Coordinate {
                x: self.state.x0,
                y: self.state.y0,
            },
            None,
        );
    }

    #[allow(clippy::too_many_arguments)]
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
        if d2 > self.state.four * self.delta2 {
            depth -= 1_u8;
            if depth > 0_u8 {
                let mut a = a0 + a1;
                let mut b = b0 + b1;
                let mut c = c0 + c1;
                let m = (a * a + b * b + c * c).sqrt();
                c = c / m;
                let phi2 = c.asin();
                let lambda2 = if (c.abs() - T::one()).abs() < self.state.epsilon
                    || (lambda0 - lambda1).abs() < self.state.epsilon
                {
                    (lambda0 + lambda1) * self.state.frac_1_2
                } else {
                    b.atan2(a)
                };

                let p = self.projection_transform.transform(&Coordinate {
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
                    || ((dx * dx2 + dy * dy2) / d2 - self.state.frac_1_2).abs()
                        > self.state.frac_1_3
                    || a0 * a1 + b0 * b1 + c0 * c1 < self.state.cos_min_distance
                {
                    a = a / m;
                    b = b / m;
                    self.resample_line_to(
                        x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b, c, depth,
                    );
                    self.state.sink.point(&Coordinate { x: x2, y: y2 }, None);

                    self.resample_line_to(
                        x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth,
                    );
                }
            }
        }
    }
}

impl<EP, PR, SC, T> Stream for Resample<PR, SC, Connected<SC, T>, T>
where
    EP: Stream<EP = EP, T = T> + Default,
    PR: Clone + Transform<T = T>,
    SC: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    #[inline]
    fn line_end(&mut self) {
        if self.state.use_line_end {
            self.line_end_default();
        } else {
            self.ring_end();
        }
    }
    #[inline]
    fn line_start(&mut self) {
        if self.state.use_line_start {
            self.line_start_default();
        } else {
            self.ring_start();
        }
    }
    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.state.point_state {
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

    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
        self.state.use_line_start = true;
    }

    fn polygon_start(&mut self) {
        self.state.sink.polygon_start();
        self.state.use_line_start = false;
    }

    #[inline]
    fn sphere(&mut self) {
        self.state.sink.sphere();
    }
}
