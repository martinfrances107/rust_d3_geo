use core::fmt::Debug;
use std::sync::mpsc::SyncSender;
use std::thread;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::abs_diff_eq;
use crate::clip::buffer::Buffer;
use crate::clip::line_elem::LineElem;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::math::EPSILON;
use crate::projection::projector_common::ChannelStatus;
use crate::projection::projector_common::Message;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::StreamMT;
use crate::stream::Unconnected;

use super::intersect::intersect;
use super::intersect::Return;

/// Circle Line.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line<STATE, T>
where
    T: CoordFloat,
{
    /// Connection State.
    state: STATE,
    /// Code for previous point.
    c0: u8,
    clean: u8, // no intersections
    pub radius: T,
    cr: T,
    not_hemisphere: bool,
    /// previous point.
    point0: Option<LineElem<T>>,
    small_radius: bool,
    /// Visibility of previous point.
    v0: bool,
    /// Visibility of first point
    v00: bool,
}
// Note Default is ONLY implemented for the unconnected state
// Added when I found it was useful for type coercion.

impl<T> Default for Line<Unconnected, T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            state: Unconnected,

            c0: 0,
            clean: 0,
            radius: T::nan(),
            cr: T::nan(),
            not_hemisphere: false,

            point0: None,
            small_radius: false,

            v0: false,

            v00: false,
        }
    }
}

impl<SINK, T> LineConnected for Line<Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    type SINK = SINK;

    #[inline]
    fn sink(&mut self) -> &mut Self::SINK {
        &mut self.state.sink
    }
}

impl<T> Bufferable for Line<Unconnected, T>
where
    T: CoordFloat,
{
    type LINE = Line<Connected<Buffer<T>>, T>;
    type T = T;

    #[inline]
    fn buffer(&mut self, buffer: Buffer<T>) -> Self::LINE {
        Line {
            state: Connected { sink: buffer },
            cr: self.cr,
            not_hemisphere: self.not_hemisphere,
            point0: self.point0,
            small_radius: self.small_radius,
            v0: self.v0,
            v00: self.v00,
            clean: self.clean,
            radius: self.radius,
            c0: self.c0,
        }
    }
}

impl<T> Connectable for Line<Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SC> = Line<Connected<SC>, T>;

    #[inline]
    fn connect<SC>(&self, sink: SC) -> Line<Connected<SC>, T> {
        // Copy Mutate.
        Line {
            state: Connected { sink },
            cr: self.cr,
            not_hemisphere: self.not_hemisphere,
            point0: self.point0,
            small_radius: self.small_radius,
            v0: self.v0,
            v00: self.v00,
            clean: self.clean,
            radius: self.radius,
            c0: self.c0,
        }
    }
}

impl<T> Line<Unconnected, T>
where
    T: CoordFloat,
{
    /// Constructor.
    ///
    /// # Panics
    /// `unwrap()` is used here but a panic will never happen as EPSILON will always be converted into T.
    #[inline]
    pub fn new(radius: T) -> Self {
        let cr = radius.cos();
        let small_radius = cr.is_sign_positive();
        let epsilon = T::from(EPSILON).unwrap();
        Self {
            state: Unconnected,
            c0: 0,
            clean: 0,
            // JS TODO optimize for this common case
            not_hemisphere: cr.abs() > epsilon,
            point0: None,
            cr,
            radius,
            small_radius,
            v0: false,
            v00: false,
        }
    }
}

impl<STATE, T> PointVisible for Line<STATE, T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn point_visible(&self, p: &Coord<T>) -> bool {
        p.x.cos() * p.y.cos() > self.cr
    }
}

/// Initial value, point is not visible or the small circle is not defined.
static CODE_NONE: u8 = 0;
/// Left of the bounding box.
static CODE_LEFT: u8 = 1;
/// Right of the bounding box.
static CODE_RIGHT: u8 = 2;
/// Below  the bounding box.
static CODE_BELOW: u8 = 4;
/// Above the bounding box.
static CODE_ABOVE: u8 = 8;

/// Generates a 4-bit vector representing the location of a point relative to
/// the small circle's bounding box.
///
/// code is only available of from connected state.
impl<S, T> Line<S, T>
where
    T: CoordFloat + FloatConst,
{
    fn code(&self, p: &Coord<T>) -> u8 {
        let lambda = p.x;
        let phi = p.y;
        let r = if self.small_radius {
            self.radius
        } else {
            T::PI() - self.radius
        };
        let mut code = CODE_NONE;
        if lambda < -r {
            code |= CODE_LEFT;
        } else if lambda > r {
            code |= CODE_RIGHT;
        }
        if phi < -r {
            code |= CODE_BELOW;
        } else if phi > r {
            code |= CODE_ABOVE;
        }

        code
    }
}

/// API clean only available once connected.
impl<SINK, T> Clean for Line<Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    #[inline]
    fn clean(&self) -> u8 {
        let b: u8 = u8::from(self.v00 && self.v0) << 1;
        self.clean | b
    }
}

impl<EP, SINK, T> Stream for Line<Connected<SINK>, T>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink().endpoint()
    }

    fn line_end(&mut self) {
        if self.v0 {
            self.state.sink().line_end();
        }
        self.point0 = None;
    }

    fn line_start(&mut self) {
        self.v00 = false;
        self.v0 = false;
        self.clean = 1;
    }

    #[allow(clippy::too_many_lines)]
    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        let mut point1 = Some(LineElem { p: *p, m: None });
        let mut point2: Option<LineElem<T>>;
        let v = self.point_visible(p);

        let c = if self.small_radius {
            if v {
                CODE_NONE
            } else {
                self.code(p)
            }
        } else if v {
            let inc = if p.x < T::zero() { T::PI() } else { -T::PI() };
            self.code(&Coord {
                x: p.x + inc,
                y: p.y,
            })
        } else {
            CODE_NONE
        };

        if self.point0.is_none() {
            self.v00 = v;
            self.v0 = v;
            if v {
                self.state.sink.line_start();
            }
        }

        if v != self.v0 {
            point2 = match intersect(
                &self.point0.unwrap(),
                &point1.unwrap(),
                self.radius.cos(),
                false,
            ) {
                Return::One(p_return) => p_return,
                Return::None => None,
                Return::False => {
                    todo!("This case is not handled by test");
                    // I think I should set point2 to None here but must test.
                }
                Return::Two(_t) => {
                    // There is a subtle bug in the javascript here two points is handles
                    // as if the second does not exits.
                    // For now just cause a panic here to see how many times it occurs.
                    panic!("Requested One or None found Two as !!");
                }
            };

            if point2.is_some()
                || abs_diff_eq(&self.point0.unwrap().p, &point2.unwrap().p)
                || abs_diff_eq(&point1.unwrap().p, &point2.unwrap().p)
            {
                point1.map_or_else(
                    || {
                        panic!("Trying to set m on a blank.");
                    },
                    |p| {
                        point1 = Some(LineElem { p: p.p, m: Some(1) });
                    },
                );
            }
        }

        if v != self.v0 {
            self.clean = 0;
            if v {
                // outside going in
                self.state.sink.line_start();
                point2 = match intersect(
                    &point1.unwrap(),
                    &self.point0.unwrap(),
                    self.cr,
                    false,
                ) {
                    Return::One(le) => le,
                    Return::Two([_p, _m]) => {
                        panic!("Silently dropping second point.");
                    }
                    Return::None => None,
                    Return::False => {
                        todo!("must cover this case.");
                    }
                };
                self.state.sink.point(&point2.unwrap().p, None);
            } else {
                // Inside going out.
                point2 = match intersect(
                    &self.point0.unwrap(),
                    &point1.unwrap(),
                    self.cr,
                    false,
                ) {
                    Return::One(le) => le,
                    Return::Two([_, _]) => {
                        panic!("Silently dropping second point.");
                    }
                    Return::None => None,
                    Return::False => {
                        todo!("must handle this case.");
                    }
                };

                self.state.sink.point(&point2.unwrap().p, Some(2));
                self.state.sink.line_end();
            }
            self.point0 = point2;
        } else if self.not_hemisphere
            && self.point0.is_some()
            && self.small_radius ^ v
        {
            // If the codes for two points are different, or are both zero,
            // and there this segment intersects with the small circle.
            if self.c0 != c || c == CODE_NONE {
                let t = intersect(
                    &point1.unwrap(),
                    &self.point0.unwrap(),
                    self.cr,
                    true,
                );
                match t {
                    // Request two received one!!
                    // This copies the behavior of the javascript original.
                    Return::False | Return::None | Return::One(_) => {}
                    Return::Two(t) => {
                        self.clean = 0;
                        if self.small_radius {
                            self.state.sink.line_start();
                            self.state.sink.point(&t[0], None);
                            self.state.sink.point(&t[1], None);
                            self.state.sink.line_end();
                        } else {
                            self.state.sink.point(&t[1], None);
                            self.state.sink.line_end();
                            self.state.sink.line_start();
                            self.state.sink.point(&t[0], Some(3_u8));
                        }
                    }
                }
            }
        }
        if v && (self.point0.is_none()
            || !abs_diff_eq(&self.point0.unwrap().p, &point1.unwrap().p))
        {
            self.state.sink.point(&point1.unwrap().p, None);
        }
        self.point0 = point1;
        self.v0 = v;
        self.c0 = c;
    }
}

impl<T> StreamMT<T> for Line<Unconnected, T>
where
    T: 'static + CoordFloat + FloatConst + Send,
{
    fn gen_stage(
        mut self,
        tx: SyncSender<Message<T>>,
        rx: std::sync::mpsc::Receiver<Message<T>>,
    ) -> std::thread::JoinHandle<ChannelStatus<T>> {
        // Stage pipelines.
        thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            let a;
            loop {
                a = match rx.recv() {
                    Ok(message) => {
                        let res_tx = match message {
                            Message::Point((p, _m)) => {
                                let mut point1 = Some(LineElem { p, m: None });
                                let mut point2: Option<LineElem<T>>;
                                let v = self.point_visible(&p);

                                let c = if self.small_radius {
                                    if v {
                                        CODE_NONE
                                    } else {
                                        self.code(&p)
                                    }
                                } else if v {
                                    let inc = if p.x < T::zero() {
                                        T::PI()
                                    } else {
                                        -T::PI()
                                    };
                                    self.code(&Coord {
                                        x: p.x + inc,
                                        y: p.y,
                                    })
                                } else {
                                    CODE_NONE
                                };

                                if self.point0.is_none() {
                                    self.v00 = v;
                                    self.v0 = v;
                                    if v {
                                        // self.state.sink.line_start();
                                        if let Err(e) =
                                            tx.send(Message::LineStart)
                                        {
                                            return ChannelStatus::Tx(e);
                                        }
                                    }
                                }

                                if v != self.v0 {
                                    point2 = match intersect(
                                        &self.point0.unwrap(),
                                        &point1.unwrap(),
                                        self.radius.cos(),
                                        false,
                                    ) {
                                        Return::One(p_return) => p_return,
                                        Return::None => None,
                                        Return::False => {
                                            todo!("This case is not handled by test");
                                            // I think I should set point2 to None here but must test.
                                        }
                                        Return::Two(_t) => {
                                            // There is a subtle bug in the javascript here two points is handles
                                            // as if the second does not exits.
                                            // For now just cause a panic here to see how many times it occurs.
                                            panic!("Requested One or None found Two as !!");
                                        }
                                    };

                                    if point2.is_some()
                                        || abs_diff_eq(
                                            &self.point0.unwrap().p,
                                            &point2.unwrap().p,
                                        )
                                        || abs_diff_eq(
                                            &point1.unwrap().p,
                                            &point2.unwrap().p,
                                        )
                                    {
                                        point1.map_or_else(
                    || {
                        panic!("Trying to set m on a blank.");
                    },
                    |p| {
                        point1 = Some(LineElem { p: p.p, m: Some(1) });
                    },
                );
                                    }
                                }

                                if v != self.v0 {
                                    self.clean = 0;
                                    if v {
                                        // outside going in
                                        // self.state.sink.line_start();
                                        if let Err(e) =
                                            tx.send(Message::LineStart)
                                        {
                                            return ChannelStatus::Tx(e);
                                        }
                                        point2 = match intersect(
                                            &point1.unwrap(),
                                            &self.point0.unwrap(),
                                            self.cr,
                                            false,
                                        ) {
                                            Return::One(le) => le,
                                            Return::Two([_p, _m]) => {
                                                panic!("Silently dropping second point.");
                                            }
                                            Return::None => None,
                                            Return::False => {
                                                todo!("must cover this case.");
                                            }
                                        };
                                        // self.state
                                        //     .sink
                                        //     .point(&point2.unwrap().p, None);
                                        if let Err(e) = tx.send(Message::Point(
                                            (point2.unwrap().p, None),
                                        )) {
                                            return ChannelStatus::Tx(e);
                                        };
                                    } else {
                                        // Inside going out.
                                        point2 = match intersect(
                                            &self.point0.unwrap(),
                                            &point1.unwrap(),
                                            self.cr,
                                            false,
                                        ) {
                                            Return::One(le) => le,
                                            Return::Two([_, _]) => {
                                                panic!("Silently dropping second point.");
                                            }
                                            Return::None => None,
                                            Return::False => {
                                                todo!("must handle this case.");
                                            }
                                        };

                                        // self.state
                                        //     .sink
                                        //     .point(&point2.unwrap().p, Some(2));
                                        if let Err(e) = tx.send(Message::Point(
                                            (point2.unwrap().p, Some(2)),
                                        )) {
                                            return ChannelStatus::Tx(e);
                                        };
                                        // self.state.sink.line_end();
                                        if let Err(e) =
                                            tx.send(Message::LineEnd)
                                        {
                                            return ChannelStatus::Tx(e);
                                        }
                                    }
                                    self.point0 = point2;
                                } else if self.not_hemisphere
                                    && self.point0.is_some()
                                    && self.small_radius ^ v
                                {
                                    // If the codes for two points are different, or are both zero,
                                    // and there this segment intersects with the small circle.
                                    if self.c0 != c || c == CODE_NONE {
                                        let t = intersect(
                                            &point1.unwrap(),
                                            &self.point0.unwrap(),
                                            self.cr,
                                            true,
                                        );
                                        match t {
                                            // Request two received one!!
                                            // This copies the behavior of the javascript original.
                                            Return::False
                                            | Return::None
                                            | Return::One(_) => {}
                                            Return::Two(t) => {
                                                self.clean = 0;
                                                if self.small_radius {
                                                    // self.state
                                                    //     .sink
                                                    //     .line_start();
                                                    if let Err(e) = tx.send(
                                                        Message::LineStart,
                                                    ) {
                                                        return ChannelStatus::Tx(e);
                                                    }
                                                    // self.state
                                                    //     .sink
                                                    //     .point(&t[0], None);
                                                    if let Err(e) =
                                                        tx.send(Message::Point(
                                                            (t[0], None),
                                                        ))
                                                    {
                                                        return ChannelStatus::Tx(e);
                                                    };
                                                    // self.state
                                                    //     .sink
                                                    //     .point(&t[1], None);
                                                    if let Err(e) =
                                                        tx.send(Message::Point(
                                                            (t[1], None),
                                                        ))
                                                    {
                                                        return ChannelStatus::Tx(e);
                                                    };
                                                    // self.state.sink.line_end();
                                                    if let Err(e) = tx
                                                        .send(Message::LineEnd)
                                                    {
                                                        return ChannelStatus::Tx(e);
                                                    }
                                                } else {
                                                    // self.state
                                                    //     .sink
                                                    //     .point(&t[1], None);
                                                    if let Err(e) =
                                                        tx.send(Message::Point(
                                                            (t[1], None),
                                                        ))
                                                    {
                                                        return ChannelStatus::Tx(e);
                                                    };
                                                    // self.state.sink.line_end();
                                                    if let Err(e) = tx
                                                        .send(Message::LineEnd)
                                                    {
                                                        return ChannelStatus::Tx(e);
                                                    }
                                                    // self.state
                                                    //     .sink
                                                    //     .line_start();
                                                    if let Err(e) = tx.send(
                                                        Message::LineStart,
                                                    ) {
                                                        return ChannelStatus::Tx(e);
                                                    }
                                                    // self.state.sink.point(
                                                    //     &t[0],
                                                    //     Some(3_u8),
                                                    // );
                                                    if let Err(e) =
                                                        tx.send(Message::Point(
                                                            (t[0], Some(3)),
                                                        ))
                                                    {
                                                        return ChannelStatus::Tx(e);
                                                    };
                                                }
                                            }
                                        }
                                    }
                                }
                                if v && (self.point0.is_none()
                                    || !abs_diff_eq(
                                        &self.point0.unwrap().p,
                                        &point1.unwrap().p,
                                    ))
                                {
                                    // self.state
                                    //     .sink
                                    //     .point(&point1.unwrap().p, None);
                                    if let Err(e) = tx.send(Message::Point((
                                        point1.unwrap().p,
                                        None,
                                    ))) {
                                        return ChannelStatus::Tx(e);
                                    };
                                }
                                self.point0 = point1;
                                self.v0 = v;
                                self.c0 = c;
                                Ok(())
                            }
                            Message::LineEnd => {
                                if self.v0 {
                                    if let Err(e) = tx.send(Message::LineEnd) {
                                        return ChannelStatus::Tx(e);
                                    }
                                }
                                self.point0 = None;
                                Ok(())
                            }
                            Message::LineStart => {
                                self.v00 = false;
                                self.v0 = false;
                                self.clean = 1;
                                Ok(())
                            }

                            Message::EndPoint(ep) => {
                                tx.send(Message::EndPoint(ep))
                            }
                            Message::PolygonStart
                            | Message::PolygonEnd
                            | Message::Sphere => {
                                // NoOp
                                Ok(())
                            }
                            Message::ShutDown => todo!(),
                            Message::ShutDownWithReturn(_ep) => {
                                todo!()
                            }
                        };
                        match res_tx {
                            Ok(()) => {
                                continue;
                            }
                            Err(e) => ChannelStatus::Tx(e),
                        }
                    }
                    Err(e) => ChannelStatus::Rx(e),
                };

                break;
            }
            a
        })
    }
}
