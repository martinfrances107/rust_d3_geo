use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::abs_diff_eq;
use crate::clip::buffer::Buffer;
use crate::clip::line_elem::LineElem;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::LineConnected;
use crate::math::EPSILON;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::ConnectedState;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::intersect::intersect;
use super::intersect::IntersectReturn;

/// Circle Line.
#[derive(Clone, Debug)]
pub struct Line<SC, STATE, T>
where
    T: CoordFloat,
{
    /// Connection State.
    state: STATE,
    /// PhantomData here soley to allow SINK to be defined in the Connecteable.
    p_sc: PhantomData<SC>,
    /// Code for previous point.
    c0: u8,
    clean: u8, // no intersections
    radius: T,
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
// Note Default is ONLY implenented for the unconnected state
// Added when I found it was useful for type corercion.

impl<RC, T> Default for Line<RC, Unconnected, T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            state: Unconnected,

            p_sc: PhantomData::<RC>,

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

impl<SINK, T> LineConnected for Line<SINK, Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    type SC = SINK;

    #[inline]
    fn sink(&mut self) -> &mut Self::SC {
        &mut self.state.sink
    }
}

impl<SC, T> Bufferable for Line<SC, Unconnected, T>
where
    T: CoordFloat,
{
    type Output = Line<Buffer<T>, Connected<Buffer<T>>, T>;
    type T = T;

    #[inline]
    fn buffer(&mut self, buffer: Buffer<T>) -> Self::Output {
        Line {
            state: Connected { sink: buffer },
            p_sc: PhantomData::<Buffer<T>>,
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

impl<SC, T> Connectable for Line<SC, Unconnected, T>
where
    SC: Clone,
    T: CoordFloat,
{
    type SC = SC;
    type Output = Line<SC, Connected<SC>, T>;

    #[inline]
    fn connect(self, sink: SC) -> Line<SC, Connected<SC>, T> {
        // Copy Mutate.
        Line {
            state: Connected { sink },
            p_sc: PhantomData::<SC>,
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

impl<SC, T> Line<SC, Unconnected, T>
where
    T: CoordFloat,
{
    /// Constructor.
    #[inline]
    pub fn new(radius: T) -> Self {
        // TODO small_radius, rc  is a shadow variables!!!
        let cr = radius.cos();
        let small_radius = cr.is_sign_positive();
        let epsilon = T::from(EPSILON).unwrap();
        Self {
            state: Unconnected,
            p_sc: PhantomData::<SC>,
            c0: 0,
            clean: 0,
            // JS TODO optimise for this common case
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

impl<SINK, T> Line<SINK, Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    // todo remove this duplicate.
    #[inline]
    fn visible(&self, p: &Coordinate<T>) -> bool {
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
/// TODO :-
/// code is only available of from connected state.
impl<SINK, T> Line<SINK, Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat + FloatConst,
{
    fn code(&self, p: &Coordinate<T>) -> u8 {
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

/// API clean only availble once connected.
impl<SINK, T> Clean for Line<SINK, Connected<SINK>, T>
where
    SINK: Clone,
    T: CoordFloat,
{
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    #[inline]
    fn clean(&self) -> u8 {
        let b: u8 = ((self.v00 && self.v0) as u8) << 1;
        self.clean | b
    }
}

impl<EP, SINK, T> Stream for Line<SINK, Connected<SINK>, T>
where
    SINK: Clone + Stream<EP = EP, T = T>,
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
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        let mut point1 = Some(LineElem { p: *p, m: None });
        let mut point2: Option<LineElem<T>>;
        let v = self.visible(p);

        let c = if self.small_radius {
            if v {
                CODE_NONE
            } else {
                self.code(p)
            }
        } else if v {
            let inc = if p.x < T::zero() { T::PI() } else { -T::PI() };
            self.code(&Coordinate {
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
                IntersectReturn::One(p_return) => p_return,
                IntersectReturn::None => None,
                IntersectReturn::False => {
                    todo!("This case is not handled by test");
                    // I think I should set point2 to None here but must test.
                }
                IntersectReturn::Two(_t) => {
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
                match point1 {
                    Some(p) => {
                        point1 = Some(LineElem { p: p.p, m: Some(1) });
                    }
                    None => {
                        panic!("Trying to set m on a blank.");
                    }
                }
            }
        }

        if v != self.v0 {
            self.clean = 0;
            if v {
                // outside going in
                self.state.sink.line_start();
                point2 = match intersect(&point1.unwrap(), &self.point0.unwrap(), self.cr, false) {
                    IntersectReturn::One(le) => le,
                    IntersectReturn::Two([_p, _m]) => {
                        panic!("Silently dropping second point.");
                    }
                    IntersectReturn::None => None,
                    IntersectReturn::False => {
                        todo!("must cover this case.");
                    }
                };
                self.state.sink.point(&point2.unwrap().p, None)
            } else {
                // Inside going out.
                point2 = match intersect(&self.point0.unwrap(), &point1.unwrap(), self.cr, false) {
                    IntersectReturn::One(le) => le,
                    IntersectReturn::Two([_, _]) => {
                        panic!("Silently dropping second point.");
                    }
                    IntersectReturn::None => None,
                    IntersectReturn::False => {
                        todo!("must handle this case.");
                    }
                };

                self.state.sink.point(&point2.unwrap().p, Some(2));
                self.state.sink.line_end();
            }
            self.point0 = point2;
        } else if self.not_hemisphere && self.point0.is_some() && self.small_radius ^ v {
            // If the codes for two points are different, or are both zero,
            // and there this segment intersects with the small circle.
            if self.c0 != c || c == CODE_NONE {
                let t = intersect(&point1.unwrap(), &self.point0.unwrap(), self.cr, true);
                match t {
                    // Request two received one!!
                    // This copies the behaviour of the javascript original.
                    IntersectReturn::False | IntersectReturn::None | IntersectReturn::One(_) => {}
                    IntersectReturn::Two(t) => {
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
        if v && (self.point0.is_none() || !abs_diff_eq(&self.point0.unwrap().p, &point1.unwrap().p))
        {
            self.state.sink.point(&point1.unwrap().p, None);
        }
        self.point0 = point1;
        self.v0 = v;
        self.c0 = c;
    }
}
