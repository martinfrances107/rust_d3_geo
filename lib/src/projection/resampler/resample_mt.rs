use core::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use geo::CoordFloat;
use geo_types::Coord;

use crate::cartesian::cartesian;
use crate::compose::Compose;
use crate::math::EPSILON;
use crate::projection::projector_common::ChannelError;
use crate::projection::projector_common::Message;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::stream::StreamMT;
use crate::Transform;

static MAXDEPTH: u8 = 16_u8; // maximum depth of subdivision

#[derive(Clone, Debug)]
enum PointState {
    Default,
    Line,
    Ring,
}

/// Resample the stream based on a given precision.
#[derive(Clone)]
pub struct ResampleMT<PR, T>
where
    T: CoordFloat,
{
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

    /// generic constants T
    epsilon: T,
    four: T,
    frac_1_2: T,
    frac_1_3: T,

    delta2: T,
    projection_transform: Compose<PR, ScaleTranslateRotate<T>>,
}

impl<PR, T> Debug for ResampleMT<PR, T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Resample")
            .field(&self.delta2)
            // .field(&self.state)
            .finish()
    }
}

impl<PR, T> ResampleMT<PR, T>
where
    T: CoordFloat,
{
    /// Returns a Resample for a given precision.
    #[inline]
    pub fn new(
        projection_transform: Compose<PR, ScaleTranslateRotate<T>>,
        delta2: T,
    ) -> Self {
        Self {
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

            // cos(minimum angular distance)
            cos_min_distance: T::from(30_f64).unwrap().to_radians().cos(),
            point_state: PointState::Default,
            use_line_start: true,
            use_line_end: true,

            // Generic constants.
            epsilon: T::from(EPSILON).unwrap(),
            four: T::from(4_f64).unwrap(),
            frac_1_2: T::from(0.5_f64).unwrap(),
            frac_1_3: T::from(1_f64 / 3_f64).unwrap(),
            delta2,
            projection_transform,
        }
    }
}

impl<PR, T> ResampleMT<PR, T>
where
    PR: Transform<T = T>,
    T: CoordFloat,
{
    fn point_default(
        &self,
        tx: &Sender<Message<T>>,
        p: &Coord<T>,
        m: Option<u8>,
    ) -> Result<(), SendError<Message<T>>> {
        let pt = self.projection_transform.transform(p);
        // self.state.sink.point(&pt, m);
        tx.send(Message::Point((pt, m)))
    }

    fn line_start_default(
        &mut self,
        tx: &Sender<Message<T>>,
    ) -> Result<(), SendError<Message<T>>> {
        self.x0 = T::nan();
        self.point_state = PointState::Line;
        // self.state.sink.line_start();
        tx.send(Message::LineStart)
    }

    fn line_end_default(
        &mut self,
        tx: &Sender<Message<T>>,
    ) -> Result<(), SendError<Message<T>>> {
        self.point_state = PointState::Default;
        // self.state.sink.line_end();
        tx.send(Message::LineEnd)
    }

    fn ring_start(
        &mut self,
        tx: &Sender<Message<T>>,
    ) -> Result<(), SendError<Message<T>>> {
        self.line_start_default(tx)?;
        self.point_state = PointState::Ring;
        self.use_line_end = false;
        Ok(())
    }

    fn ring_point(
        &mut self,
        tx: &Sender<Message<T>>,
        p: &Coord<T>,
    ) -> Result<(), SendError<Message<T>>> {
        self.lambda00 = p.x;

        self.x00 = self.x0;
        self.y00 = self.y0;
        self.a00 = self.a0;
        self.b00 = self.b0;
        self.c00 = self.c0;
        self.point_state = PointState::Line;
        self.line_point(
            tx,
            &Coord {
                x: self.lambda00,
                y: p.y,
            },
        )
    }

    fn ring_end(
        &mut self,
        tx: &Sender<Message<T>>,
    ) -> Result<(), SendError<Message<T>>> {
        self.resample_line_to(
            tx,
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
        )?;

        self.use_line_end = true;

        // self.sink.line_end();
        tx.send(Message::LineEnd)
    }

    fn line_point(
        &mut self,
        tx: &Sender<Message<T>>,
        p: &Coord<T>,
    ) -> Result<(), SendError<Message<T>>> {
        let c = cartesian(p);
        let p_transformed = self.projection_transform.transform(p);
        self.resample_line_to(
            tx,
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
        )?;
        self.x0 = p_transformed.x;
        self.y0 = p_transformed.y;
        self.lambda0 = p.x;
        self.a0 = c[0];
        self.b0 = c[1];
        self.c0 = c[2];
        tx.send(Message::Point((
            Coord {
                x: self.x0,
                y: self.y0,
            },
            None,
        )))
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::similar_names)]
    fn resample_line_to(
        &mut self,
        tx: &Sender<Message<T>>,
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
    ) -> Result<(), SendError<Message<T>>> {
        let mut depth = depth_p;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d2 = dx * dx + dy * dy;
        if d2 > self.four * self.delta2 {
            depth -= 1_u8;
            if depth > 0_u8 {
                let mut a = a0 + a1;
                let mut b = b0 + b1;
                let mut c = c0 + c1;
                let m = (a * a + b * b + c * c).sqrt();
                c = c / m;
                let phi2 = c.asin();
                let lambda2 = if (c.abs() - T::one()).abs() < self.epsilon
                    || (lambda0 - lambda1).abs() < self.epsilon
                {
                    (lambda0 + lambda1) * self.frac_1_2
                } else {
                    b.atan2(a)
                };

                let p = self.projection_transform.transform(&Coord {
                    x: lambda2,
                    y: phi2,
                });

                let x2 = p.x;
                let y2 = p.y;
                let dx2 = x2 - x0;

                let dy2 = y2 - y0;
                let dz = dy * dx2 - dx * dy2;
                // Three conditions :-
                // perpendicular projected distance
                // midpoint close to an end
                // angular distance
                if dz * dz / d2 > self.delta2
                    || ((dx * dx2 + dy * dy2) / d2 - self.frac_1_2).abs()
                        > self.frac_1_3
                    || a0 * a1 + b0 * b1 + c0 * c1 < self.cos_min_distance
                {
                    a = a / m;
                    b = b / m;
                    self.resample_line_to(
                        tx, x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b,
                        c, depth,
                    )?;
                    // self.state.sink.point(&Coord { x: x2, y: y2 }, None);
                    tx.send(Message::Point((Coord { x: x2, y: y2 }, None)))?;

                    self.resample_line_to(
                        tx, x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1,
                        c1, depth,
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl<PR, T> StreamMT<T> for ResampleMT<PR, T>
where
    PR: 'static + Transform<T = T> + Send,
    T: 'static + CoordFloat + Send,
{
    fn gen_stage(
        mut self,
        tx: Sender<Message<T>>,
        rx: Receiver<Message<T>>,
    ) -> JoinHandle<ChannelError<T>> {
        // Stage pipelines.
        thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            let a;
            loop {
                a = match rx.recv() {
                    Ok(message) => {
                        let res_tx: Result<(), SendError<Message<T>>> =
                            match message {
                                Message::Point((p, m)) => {
                                    match self.point_state {
                                        PointState::Default => {
                                            self.point_default(&tx, &p, m)
                                        }
                                        PointState::Line => {
                                            self.line_point(&tx, &p)
                                        }
                                        PointState::Ring => {
                                            self.ring_point(&tx, &p)
                                        }
                                    }
                                }
                                Message::LineEnd => {
                                    if self.use_line_end {
                                        self.line_end_default(&tx)
                                    } else {
                                        self.ring_end(&tx)
                                    }
                                }
                                Message::LineStart => {
                                    if self.use_line_start {
                                        self.line_start_default(&tx)
                                    } else {
                                        self.ring_start(&tx)
                                    }
                                }
                                Message::PolygonStart => {
                                    if let Err(e) =
                                        tx.send(Message::PolygonStart)
                                    {
                                        Err(e)
                                    } else {
                                        self.use_line_start = false;
                                        Ok(())
                                    }
                                }
                                Message::PolygonEnd => {
                                    match tx.send(Message::PolygonEnd) {
                                        Ok(()) => {
                                            self.use_line_start = true;
                                            Ok(())
                                        }
                                        Err(e) => Err(e),
                                    }
                                }
                                Message::EndPoint(_) | Message::Sphere => {
                                    tx.send(message)
                                }
                            };
                        match res_tx {
                            Ok(()) => {
                                continue;
                            }
                            Err(e) => ChannelError::Tx(e),
                        }
                    }
                    Err(e) => ChannelError::Rx(e),
                };

                break;
            }
            a
        })
    }
}
