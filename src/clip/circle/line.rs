use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use super::intersect::intersect;
use super::intersect::IntersectReturn;

use crate::clip::line_elem::LineElem;
use crate::clip::Clean;
use crate::clip::CleanState;
use crate::clip::Line as LineTrait;
use crate::math::EPSILON;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

/// Circle Line.
#[derive(Copy, Clone, Debug)]
pub struct Line<T>
where
    T: CoordFloat,
{
    c0: u8,            // code for previous point
    clean: CleanState, // no intersections
    radius: T,
    cr: T,
    not_hemisphere: bool,
    point0: Option<LineElem<T>>, // previous point
    small_radius: bool,
    v0: bool,  // visibility of previous point
    v00: bool, // visibility of first point
}

impl<T> LineTrait for Line<T> where T: CoordFloat {}

impl<T> Line<T>
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
            c0: 0,
            clean: CleanState::IntersectionsOrEmpty,
            not_hemisphere: cr.abs() > epsilon,
            point0: None,
            cr,
            radius,
            small_radius,
            v0: false,
            v00: false,
        }
    }

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
impl<T> Line<T>
where
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

impl<T> Clean for Line<T>
where
    T: CoordFloat,
{
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    #[inline]
    fn clean(&self) -> CleanState {
        if self.v00 && self.v0 {
            CleanState::IntersectionsRejoin
        } else {
            self.clean
        }
    }
}

impl<EP, SINK, T> Stream for StreamNode<EP, Line<T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    SINK: Stream<EP = EP, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    type T = T;
    type EP = EP;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.sink.get_endpoint()
    }

    fn line_start(&mut self) {
        self.raw.v00 = false;
        self.raw.v0 = false;
        self.raw.clean = CleanState::NoIntersections;
    }

    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        let mut point1 = Some(LineElem { p: *p, m: None });
        let mut point2: Option<LineElem<T>>;
        let v = self.raw.visible(p);

        let c = if self.raw.small_radius {
            if v {
                CODE_NONE
            } else {
                self.raw.code(p)
            }
        } else if v {
            let inc = if p.x < T::zero() { T::PI() } else { -T::PI() };
            self.raw.code(&Coordinate {
                x: p.x + inc,
                y: p.y,
            })
        } else {
            CODE_NONE
        };

        // let mut s = self.sink;
        if self.raw.point0.is_none() {
            self.raw.v00 = v;
            self.raw.v0 = v;
            if v {
                self.sink.line_start();
            }
        }

        if v != self.raw.v0 {
            point2 = match intersect(
                &self.raw.point0.unwrap(),
                &point1.unwrap(),
                self.raw.radius.cos(),
                false,
            ) {
                IntersectReturn::One(p_return) => p_return,
                IntersectReturn::None => None,
                IntersectReturn::False => {
                    todo!("This case is not handled by test");
                    // I think I should set point2 to None here buy must test.
                }
                IntersectReturn::Two(_t) => {
                    // There is a subtle bug in the javascript here two points is handles
                    // as if the second does not exits.
                    // For now just cause a panic here to see how many times it occurs.
                    panic!("Requested One or None found Two as !!");
                }
            };
            let epsilon = T::from(EPSILON).unwrap();
            if point2.is_some()
                || self
                    .raw
                    .point0
                    .unwrap()
                    .p
                    .abs_diff_eq(&point2.unwrap().p, epsilon)
                || point1.unwrap().p.abs_diff_eq(&point2.unwrap().p, epsilon)
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

        if v != self.raw.v0 {
            self.raw.clean = CleanState::IntersectionsOrEmpty;
            if v {
                // outside going in
                self.sink.line_start();
                point2 = match intersect(
                    &point1.unwrap(),
                    &self.raw.point0.unwrap(),
                    self.raw.cr,
                    false,
                ) {
                    IntersectReturn::One(le) => le,
                    IntersectReturn::Two([_p, _m]) => {
                        panic!("Silently dropping second point.");
                    }
                    IntersectReturn::None => None,
                    IntersectReturn::False => {
                        todo!("must cover this case.");
                    }
                };
                self.sink.point(&point2.unwrap().p, None)
            } else {
                // Inside going out.
                point2 = match intersect(
                    &self.raw.point0.unwrap(),
                    &point1.unwrap(),
                    self.raw.cr,
                    false,
                ) {
                    IntersectReturn::One(le) => le,
                    IntersectReturn::Two([_, _]) => {
                        panic!("Silently dropping second point.");
                    }
                    IntersectReturn::None => None,
                    IntersectReturn::False => {
                        todo!("must handle this case.");
                    }
                };

                self.sink.point(&point2.unwrap().p, Some(2));
                self.sink.line_end();
            }
            self.raw.point0 = point2;
        } else if self.raw.not_hemisphere && self.raw.point0.is_some() && self.raw.small_radius ^ v
        {
            // If the codes for two points are different, or are both zero,
            // and there this segment intersects with the small circle.
            if self.raw.c0 != c || c == CODE_NONE {
                let t = intersect(
                    &point1.unwrap(),
                    &self.raw.point0.unwrap(),
                    self.raw.cr,
                    true,
                );
                match t {
                    IntersectReturn::False => {
                        // None found
                    }
                    IntersectReturn::None => {
                        // do nothing.
                    }
                    IntersectReturn::One(_) => {
                        panic!("Requeted two received one or none.");
                    }
                    IntersectReturn::Two(t) => {
                        self.raw.clean = CleanState::IntersectionsOrEmpty;
                        if self.raw.small_radius {
                            self.sink.line_start();
                            self.sink.point(&t[0], None);
                            self.sink.point(&t[1], None);
                            self.sink.line_end();
                        } else {
                            self.sink.point(&t[1], None);
                            self.sink.line_end();
                            self.sink.line_start();
                            self.sink.point(&t[0], Some(3_u8));
                        }
                    }
                }
            }
        }
        if v && (self.raw.point0.is_none()
            || !self
                .raw
                .point0
                .unwrap()
                .p
                .abs_diff_eq(&point1.unwrap().p, T::from(EPSILON).unwrap()))
        {
            self.sink.point(&point1.unwrap().p, None);
        }
        self.raw.point0 = point1;
        self.raw.v0 = v;
        self.raw.c0 = c;
    }
    fn line_end(&mut self) {
        if self.raw.v0 {
            self.sink.line_end();
        }
        self.raw.point0 = None;
    }
}
